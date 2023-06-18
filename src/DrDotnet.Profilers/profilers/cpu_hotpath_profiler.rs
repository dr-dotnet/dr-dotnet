use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

use crate::api::*;
use crate::api::ffi::{ FunctionID, HRESULT };
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{NameResolver, TreeNode };

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
            name: "CPU Hotpath Profiler".to_owned(),
            description: "Lists CPU hotpaths.".to_owned(),
            is_released: true,
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
                }
            ],
            ..std::default::Default::default()
        }
    }
}

impl CpuHotpathProfiler {

    fn build_callstacks(profiler_info: ClrProfilerInfo, mut stacks: std::sync::MutexGuard<HashMap<Vec<FunctionID>, usize>>)
    {
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {

            let mut function_ids = Vec::<FunctionID>::new();

            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let function_ids_ptr_c = &function_ids as *const Vec<FunctionID> as *mut std::ffi::c_void;

            let _ =  pinfo.do_stack_snapshot(
                managed_thread_id,
                crate::utils::stack_snapshot_callback,
                ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT,
                function_ids_ptr_c,
                std::ptr::null(), 0);

            if function_ids.is_empty() { continue }

            // Reverse the stack to get the callstack
            // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-dostacksnapshot-method#remarks
            function_ids.reverse();

            stacks.entry(function_ids)
                .and_modify(|count| { *count = *count + 1 })
                .or_insert(1);
        }
    }

    fn print_html(clr: &ClrProfilerInfo, tree: &TreeNode<FunctionID, usize>, samples: usize, report: &mut Report)
    {
        let node_samples = tree.get_inclusive_value();
        let node_incl_perc = 100f64 * node_samples as f64 / samples as f64;
        
        if tree.key == 0 {
            report.write_line(format!("<li><span>{node_incl_perc}</span><code>unmanaged</code></li>"));
            return;
        }

        // Name resolution could be cached
        let mut class_name = clr.get_full_method_name(tree.key);
        let escaped_class_name = html_escape::encode_text(&mut class_name);

        let has_children = tree.children.len() > 0;
        if has_children {
            report.write_line(format!("<details><summary><span>{:.2} %</span><code>{escaped_class_name}</code></summary>", node_incl_perc));
            report.write_line(format!("<ul>"));
            for child in &tree.children {
                CpuHotpathProfiler::print_html(clr, child, samples, report);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li><span>{:.2} %</span><code>{escaped_class_name}</code></li>", node_incl_perc));
        }
    }
}

impl CorProfilerCallback for CpuHotpathProfiler {}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler {
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let clr = self.clr().clone();
        let session_info = self.session_info().clone();

        let time_interval_ms = self.session_info().get_parameter::<u64>("time_interval_ms").unwrap();
        let duration_seconds = self.session_info().get_parameter::<u64>("duration_seconds").unwrap();

        std::thread::spawn(move || {

            let stacks: Arc<Mutex<HashMap<Vec<FunctionID>, usize>>> = Arc::new(Mutex::new(HashMap::new()));

            let iterations = 1000 * duration_seconds / time_interval_ms;
            for _ in 0..iterations {
                std::thread::sleep(std::time::Duration::from_millis(time_interval_ms));

                // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
                if clr.suspend_runtime().is_ok() {

                    let k = stacks.lock().unwrap();
                    CpuHotpathProfiler::build_callstacks(clr.clone(), k);

                    if clr.resume_runtime().is_err() {
                        error!("Can't resume runtime!");
                    }
                } else {
                    error!("Can't suspend runtime!");
                }
            }

            let mut report_html = session_info.create_report("stacks.html".to_owned());
            let k = stacks.lock().unwrap();
            
            info!("Building tree");
            let mut tree = TreeNode::build_from_sequences(&k, 0);
            let samples = tree.get_inclusive_value();

            info!("Sorting tree");
            tree.sort_by(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));
            
            info!("Printing tree");
            for x in tree.children.iter() {
                CpuHotpathProfiler::print_html(&clr, &x, samples, &mut report_html);
            }
            
            clr.request_profiler_detach(3000)
        });

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}