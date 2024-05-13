use dashmap::DashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use crate::api::ffi::{FunctionID, ThreadID, HRESULT};
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
                    value: "10".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Time Interval".to_owned(),
                    key: "time_interval_ms".to_owned(),
                    description: "Time interval between two samples in milliseconds".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "40".to_owned(),
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
                    name: "Maximum stacks to display".to_owned(),
                    key: "max_stacks".to_owned(),
                    description: "The maximum number of stacks to display".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "100".to_owned(),
                    ..std::default::Default::default()
                }
            ],
            ..std::default::Default::default()
        };
    }
}

#[derive(Default)]
pub struct CpuHotpathStackSnapshotCallbackReceiver {
    method_ids: Vec<FunctionID>,
    hasher: DefaultHasher,
}

impl StackSnapshotCallbackReceiver for CpuHotpathStackSnapshotCallbackReceiver {
    type AssociatedType = Self;

    fn callback(&mut self, method_id: FunctionID, instruction_pointer: usize, _frame_info: usize, _context: &[u8]) {
        // Filter out unmanaged stack frames
        if method_id == 0 {
            return;
        }
        self.method_ids.push(method_id);
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
        tree: &mut TreeNode<FunctionID, usize>,
        filter_suspended_threads: bool,
        caller_to_callee: bool,
    ) {
        info!("Starts building callstacks");
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {
            let mut stack_snapshot_receiver = CpuHotpathStackSnapshotCallbackReceiver::default();

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

        let mut threads_by_context_hash = DashMap::<ThreadID, u64>::new();
        let mut tree = TreeNode::<FunctionID, usize>::new(0);
        let iterations = 1000 * duration_seconds / time_interval_ms;
        for _ in 0..iterations {
            std::thread::sleep(std::time::Duration::from_millis(time_interval_ms));

            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            if clr.suspend_runtime().is_ok() {
                info!("Suspend runtime");
                Self::build_callstacks(clr.clone(), &mut threads_by_context_hash, &mut tree, filter_suspended_threads, caller_to_callee);

                if clr.resume_runtime().is_err() {
                    error!("Can't resume runtime!");
                }
            } else {
                error!("Can't suspend runtime!");
            }
        }

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
        report.write_line(format!(
            "<h3>Hotpaths <small class=\"text-muted\">{}</small></h3>",
            if caller_to_callee { "Callers to Callees" } else { "Callees to Callers" }
        ));
        report.write_line(format!("<li>{} samples</li>", total_samples));
        report.write_line(format!("<li>{} roots</li>", tree.children.len()));
        tree.children.iter().for_each(|node| Self::print_html(&clr, &node, &mut report, total_samples));

        if let Err(e) = clr.request_profiler_detach(3000) {
            error!("Could not detach for reason: {:?}", e);
        }
    }

    fn print_html(clr: &ClrProfilerInfo, node: &TreeNode<usize, usize>, report: &mut Report, total_samples: usize) {
        let percentage = 100f64 * node.get_inclusive_value() as f64 / total_samples as f64;

        let mut method_name: String = clr.get_full_method_name(node.key);
        let escaped_class_name = html_escape::encode_text(&mut method_name);

        let has_children = node.children.len() > 0;

        if has_children {
            report.write_line(format!(
                "<details><summary><span>{percentage:.2} %</span><code>{escaped_class_name}</code></summary>"
            ));
            report.write_line(format!("<ul>"));
            for child in &node.children {
                Self::print_html(clr, child, report, total_samples);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li><span>{percentage:.2} %</span><code>{escaped_class_name}</code></li>"));
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
