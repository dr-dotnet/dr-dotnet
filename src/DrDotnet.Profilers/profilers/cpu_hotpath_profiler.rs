use dashmap::DashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use crate::api::ffi::{ClassID, FunctionID, ThreadID, COR_PRF_FRAME_INFO, HRESULT};
use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::rust_protobuf_protos::interop::*;
use crate::session::Report;
use crate::utils::{NameResolver, StackSnapshotCallbackReceiver, TreeNode};

#[derive(Default)]
pub struct CpuHotpathProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
}

impl Profiler for CpuHotpathProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-A485B2056E71".to_owned(),
            name: "List CPU hotpaths".to_owned(),
            description: "Capture callstacks every X ms and for a given duration and with minimal overhead, and then sort and list hotpaths in a tree view.".to_owned(),
            parameters: vec![
                ProfilerParameter {
                    name: "Duration".to_owned(),
                    key: "duration_seconds".to_owned(),
                    description: "The profiling duration in seconds".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "30".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Time Interval".to_owned(),
                    key: "time_interval_ms".to_owned(),
                    description: "Time interval between two samples in milliseconds".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "20".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Maximum stacks to display".to_owned(),
                    key: "max_stacks".to_owned(),
                    description: "The maximum number of stacks to display".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "20".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Filter Suspended Threads".to_owned(),
                    key: "filter_suspended_threads".to_owned(),
                    description: "If set, the profiler will attempt to detect suspended thread and filter them out from the analysis to only focus on working threads, hereby improving accuracy on actual CPU hotpaths".to_owned(),
                    type_: ParameterType::BOOLEAN.into(),
                    value: "true".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Caller To Callee".to_owned(),
                    key: "caller_to_callee".to_owned(),
                    description: "If set, the output will display callers first and callees as children in the tree representation".to_owned(),
                    type_: ParameterType::BOOLEAN.into(),
                    value: "false".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Try Resolve Generics".to_owned(),
                    key: "try_resolve_generics".to_owned(),
                    description: "[Experimental] If set, try resolve generic arguments. This may not always work and might incurr additional overhead.".to_owned(),
                    type_: ParameterType::BOOLEAN.into(),
                    value: "false".to_owned(),
                    ..std::default::Default::default()
                }
            ],
            ..std::default::Default::default()
        };
    }
}

pub struct CpuHotpathStackSnapshotCallbackReceiver {
    method_ids: Vec<(FunctionID, ClassID)>,
    hasher: DefaultHasher,
    clr: ClrProfilerInfo,
    try_resolve_generics: bool,
}

impl CpuHotpathStackSnapshotCallbackReceiver {
    fn new(clr: ClrProfilerInfo, try_resolve_generics: bool) -> Self {
        CpuHotpathStackSnapshotCallbackReceiver {
            method_ids: Vec::new(),
            hasher: DefaultHasher::new(),
            clr: clr,
            try_resolve_generics: try_resolve_generics,
        }
    }
}

impl StackSnapshotCallbackReceiver for CpuHotpathStackSnapshotCallbackReceiver {
    type AssociatedType = Self;

    fn callback(&mut self, method_id: FunctionID, instruction_pointer: usize, frame_info: COR_PRF_FRAME_INFO, _context: &[u8]) {
        // Filter out unmanaged stack frames
        if method_id == 0 {
            return;
        }

        let class_id = self.try_resolve_generics
            .then(|| self.clr.get_function_info_2(method_id, frame_info).ok())
            .flatten()
            .map_or(0, |info| info.class_id);

        self.method_ids.push((method_id, class_id));
        
        // Detect suspended threads appart from actual working threads
        // Inspired from: https://www.usenix.org/legacy/publications/library/proceedings/coots99/full_papers/liang/liang_html/node10.html
        // Not sure which is the best approach between utilizing the instruction pointer or the context
        self.hasher.write_usize(instruction_pointer);
        //self.hasher.write(context);
    }
}

impl CpuHotpathProfiler {
    fn build_callstacks(
        profiler_info: ClrProfilerInfo,
        threads: &mut DashMap<ThreadID, u64>,
        tree: &mut TreeNode<(FunctionID, ClassID), usize>,
        filter_suspended_threads: bool,
        caller_to_callee: bool,
        try_resolve_generics: bool
    ) {
        debug!("Starts building callstacks");
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {
            let mut stack_snapshot_receiver = CpuHotpathStackSnapshotCallbackReceiver::new(pinfo.clone(), try_resolve_generics);

            stack_snapshot_receiver.do_stack_snapshot(pinfo.clone(), managed_thread_id, false);

            if filter_suspended_threads {
                let hash = stack_snapshot_receiver.hasher.finish();

                let mut ignore_thread = true;
                threads
                    .entry(managed_thread_id)
                    .and_modify(|existing_value| {
                        // Ignore the thread if the stack context hash is unchanged
                        ignore_thread = *existing_value == hash;
                        *existing_value = hash;
                    })
                    .or_insert(hash);

                if ignore_thread {
                    continue;
                }
            }

            // Add (reversed) callstack into tree
            let node = if caller_to_callee {
                tree.add_sequence(stack_snapshot_receiver.method_ids.into_iter().rev())
            } else {
                tree.add_sequence(stack_snapshot_receiver.method_ids.into_iter())
            };

            // Increment count for callstack occurrence
            node.value = match node.value {
                Some(value) => Some(value + 1),
                None => Some(1),
            };
        }
    }

    fn profile(session_info: SessionInfo, clr: ClrProfilerInfo) {
        let time_interval_ms = session_info.get_parameter::<u64>("time_interval_ms").unwrap();
        let duration_seconds = session_info.get_parameter::<u64>("duration_seconds").unwrap();
        let filter_suspended_threads = session_info.get_parameter::<bool>("filter_suspended_threads").unwrap();
        let caller_to_callee = session_info.get_parameter::<bool>("caller_to_callee").unwrap();
        let try_resolve_generics = session_info.get_parameter::<bool>("try_resolve_generics").unwrap();

        let mut threads_by_context_hash = DashMap::<ThreadID, u64>::new();
        let mut tree = TreeNode::<(FunctionID, ClassID), usize>::new((0, 0));
        let iterations = 1000 * duration_seconds / time_interval_ms;
        for _ in 0..iterations {
            std::thread::sleep(std::time::Duration::from_millis(time_interval_ms));

            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            match clr.suspend_runtime() {
                Ok(_) => {
                    debug!("Suspend runtime");
                    Self::build_callstacks(clr.clone(), &mut threads_by_context_hash, &mut tree, filter_suspended_threads, caller_to_callee, try_resolve_generics);

                    if clr.resume_runtime().is_err() {
                        error!("Can't resume runtime!");
                    }
                }
                Err(e) => {
                    error!("Can't suspend runtime! {:?}", e);
                }
            }
        }

        info!("Printing tree");

        let total_samples: usize = tree.get_inclusive_value();

        // Sort by descending inclusive count (hotpaths first)
        tree.sort_by(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));

        let max_stacks = session_info.get_parameter::<u64>("max_stacks").unwrap() as usize;

        // Truncate tree to max_stacks to avoid too large reports
        if tree.children.len() > max_stacks {
            tree.children.truncate(max_stacks);
        }

        // Write tree into HTML report
        let mut report = session_info.create_report("cpu_hotpaths.html".to_owned());
        report.write_line("<h2>Hotpaths</h2>".to_owned());
        report.write_line(format!("<h3>{} Tree</h3>", if caller_to_callee { "Callers to Callees" } else { "Callees to Callers" }));
        report.write_line(format!("<h4>{} samples of {} roots</h4>", total_samples, tree.children.len()));
        
        tree.children.iter().for_each(|node| Self::print_html(&clr, &node, &mut report, total_samples));

        if let Err(e) = clr.request_profiler_detach(3000) {
            error!("Could not detach for reason: {:?}", e);
        }
    }

    fn print_html(clr: &ClrProfilerInfo, node: &TreeNode<(FunctionID, ClassID), usize>, report: &mut Report, total_samples: usize) {
        let percentage_exclusive = 100f64 * node.value.unwrap_or_default() as f64 / total_samples as f64;
        let percentage_inclusive = 100f64 * node.get_inclusive_value() as f64 / total_samples as f64;

        let mut method_name: String = clr.get_full_method_name(node.key.0, node.key.1);
        let escaped_class_name = html_escape::encode_text(&mut method_name);

        let has_children = node.children.len() > 0;

        let line: String = format!("<code>{escaped_class_name}</code> \
            <div class=\"chip\"><span>{percentage_inclusive:.2} %</span><i class=\"material-icons\">radio_button_checked</i></div> \
            <div class=\"chip\"><span>{percentage_exclusive:.2} %</span><i class=\"material-icons\">radio_button_unchecked</i></div>");

        if has_children {
            report.write_line(format!(
                "<details><summary>{line}</summary>"
            ));
            report.write_line(format!("<ul>"));
            for child in &node.children {
                Self::print_html(clr, child, report, total_samples);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li>{line}</li>"));
        }
    }
}

impl CorProfilerCallback for CpuHotpathProfiler {}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {
        self.init(
            ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT,
            None,
            profiler_info,
            client_data,
            client_data_length,
        )
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let clr: ClrProfilerInfo = self.clr().clone();
        let session_info: SessionInfo = self.session_info().clone();

        // Run profiling in separate thread
        std::thread::spawn(move || CpuHotpathProfiler::profile(session_info, clr));

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), HRESULT> {

        self.session_info.finish();
        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}
