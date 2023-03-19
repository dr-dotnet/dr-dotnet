use std::cmp::min;
use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ Ordering, AtomicBool, AtomicIsize };
use itertools::Itertools;
use protobuf::well_known_types::timestamp::Timestamp;

use crate::api::*;
use crate::api::ffi::{FunctionID, HRESULT, ThreadID};
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::TreeNode;

const PADDING: usize = 5;

#[derive(Default)]
pub struct ParallelStacksProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    sequences: Arc<Mutex<HashMap<Vec<FunctionID>, usize>>>
}

impl Profiler for ParallelStacksProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "9404d16c-b49e-11ed-afa1-0242ac120003".to_owned(),
            name: "Parallel stacks Profiler".to_owned(),
            description: "Display a view of aggregated call stacks. ".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }
}

impl ParallelStacksProfiler {

    fn build_sequences(profiler_info: ClrProfilerInfo, mut sequences: std::sync::MutexGuard<HashMap<Vec<FunctionID>, usize>>)
    {
        info!("Starts building sequences");
        let pinfo = profiler_info.clone();

        for managed_thread_id in pinfo.enum_threads().unwrap() {

            let mut method_ids = Vec::<ffi::FunctionID>::new();

            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let method_ids_ptr_c = &method_ids as *const Vec<ffi::FunctionID> as *mut std::ffi::c_void;

            let _ =  pinfo.do_stack_snapshot(
                managed_thread_id,
                crate::utils::stack_snapshot_callback,
                ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT,
                method_ids_ptr_c,
                std::ptr::null(), 0);
            
            let count = sequences.entry(method_ids).or_insert(0);
            *count += 1;
        }
    }

    fn print(&self, tree: &TreeNode<FunctionID>, depth: usize, report: &mut Report)
    {
        let tabs = str::repeat(" ", PADDING * depth);
        let new_line = format!("\r\n{tabs}");

        let thread_count = format!("{:>PADDING$} ", tree.inclusive);
        
        report.write(new_line.as_str());
        report.write(thread_count);
        let frame = unsafe { self.clr().get_full_method_name(tree.value) };
        report.write(frame);

        if tree.exclusive > 0 {
            report.write(new_line.as_str());
            report.write(format!("~~~~ {}", tree.exclusive))
        }
        
        for child in &tree.children {

            if depth == 1 {
                report.write_line(format!("\n\n{}", str::repeat("_", 50)));
            }
            
            let child_depth = if child.inclusive != tree.inclusive { depth + 1} else { depth};
            
            self.print(child, child_depth, report);
        }
    }
}

impl CorProfilerCallback for ParallelStacksProfiler {}

impl CorProfilerCallback2 for ParallelStacksProfiler {}

impl CorProfilerCallback3 for ParallelStacksProfiler {
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        
        let profiler_info = self.clr().clone();
        let sequences = self.sequences.clone();
        
        let thread_handle = std::thread::spawn(move || {
            
            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            if profiler_info.suspend_runtime().is_ok() {
                
                let k = sequences.lock().unwrap();
                ParallelStacksProfiler::build_sequences(profiler_info.clone(), k);
                
                if profiler_info.resume_runtime().is_err() {
                    error!("Can't resume runtime!");
                }
            } else {
                error!("Can't suspend runtime!");
            }
        });
        
        if thread_handle.join().is_err() {
            error!("Can't wait for the thread to finish!");
        }

        let mut sequences = self.sequences.lock().unwrap();

        // METHOD_IDS ARE STACKED, we need to reverse them to read from top to bottom instead of the opposite
        let keys: Vec<_> = sequences.keys().cloned().collect();
        for key in keys {
            if let Some(value) = sequences.remove(&key) {
                let mut new_key = key;
                new_key.reverse();
                sequences.insert(new_key, value);
            }
        }
        
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        tree.sort_by(&|a, b| b.inclusive.cmp(&a.inclusive));

        let mut report = self.session_info.create_report("summary.md".to_owned());
        report.write_line(format!("# Merged Callstacks"));
        self.print(&tree, 0, &mut report);
        
        self.clr().request_profiler_detach(3000)
    }
}

impl CorProfilerCallback4 for ParallelStacksProfiler {}
impl CorProfilerCallback5 for ParallelStacksProfiler {}
impl CorProfilerCallback6 for ParallelStacksProfiler {}
impl CorProfilerCallback7 for ParallelStacksProfiler {}
impl CorProfilerCallback8 for ParallelStacksProfiler {}
impl CorProfilerCallback9 for ParallelStacksProfiler {}