use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
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
use crate::utils::tree::TreeNode;

const PADDING: usize = 5;
/// if < 0 will print all thread ids
const NB_THREAD_IDS_TO_PRINT: usize = 4;

#[derive(Default)]
pub struct ParallelStacksProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    sequences: Arc<Mutex<HashMap<Vec<FunctionID>, Threads>>>
}

// Required to wrap Vec<ThreadID> in order to implement AddAssign
#[derive(Clone, Default, Debug)]
pub struct Threads(Vec<ThreadID>);

// Implement AddAssign for get_inclusive_value to be usable
impl AddAssign<&Threads> for Threads {
    fn add_assign(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }
}

impl Display for Threads {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        let thread_ids = &self.0;
        
        let count = thread_ids.len();
        let limit = min(count, NB_THREAD_IDS_TO_PRINT);

        if limit < 0 {
            let str = thread_ids.iter().map(|k| format!("{k}")).collect::<Vec<String>>().join(",");
            return write!(f, "{str}");
        }

        let mut result = thread_ids.get(..limit).unwrap_or_default().iter().map(|k| format!("{k}")).collect::<Vec<String>>().join(",");
        if count > limit {
            result += "...";
        }
        write!(f, "{result}")
    }
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

    fn build_sequences(profiler_info: ClrProfilerInfo, mut sequences: std::sync::MutexGuard<HashMap<Vec<FunctionID>, Threads>>)
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
            
            sequences.entry(method_ids)
                .and_modify(|f| f.0.push(managed_thread_id))
                .or_insert(Threads(vec![managed_thread_id]));
        }
    }

    fn print(&self, tree: &TreeNode<FunctionID, Threads>, depth: usize, report: &mut Report)
    {
        let tabs = str::repeat(" ", PADDING * depth);
        let new_line = format!("\r\n{tabs}");

        let inclusive = &tree.get_inclusive_value();
        let thread_count = format!("{:>PADDING$} ", inclusive.0.len());
        
        report.write(new_line.as_str());
        report.write(thread_count);
        let frame = unsafe { self.clr().get_full_method_name(tree.key) };
        report.write(frame);

        if let Some(value) = &tree.value {
            report.write(new_line.as_str());
            report.write(format! ("~~~~ {}", value))
        }
        
        for child in &tree.children {
            
            let child_depth = if inclusive.0.len() != child.get_inclusive_value().0.len() { depth + 1} else { depth};
            
            self.print(child, child_depth, report);
        }
    }

    fn print_html(&self, tree: &TreeNode<FunctionID, Threads>, is_same_level: bool, report: &mut Report)
    {
        let inclusive = &tree.get_inclusive_value();
        let nb_threads = inclusive.0.len();
        let frame = unsafe { self.clr().get_full_method_name(tree.key) };

        if !is_same_level {
            report.write(format!("\n<details><summary><span>{nb_threads}</span>{frame}</summary>"));
        } else {
            report.write(format!("\n<li><span>{nb_threads}</span>{frame}</li>"));
        }

        for child in &tree.children {
            // pstacks style, less nested
            // let has_same_alignment = nb_threads == child.get_inclusive_value().0.len();
            // nest even if child has same count of threads because of multiple children
            let has_same_alignment = (child.children.is_empty() || child.children.len() == 1) && nb_threads == child.get_inclusive_value().0.len();
            
            if has_same_alignment && !is_same_level {
                report.write(format!("\n<ul>\n"));
            }
            self.print_html(child, has_same_alignment, report);

            if has_same_alignment && !is_same_level {
                report.write(format!("\n</ul>\n"));
            }
        }

        if !is_same_level {
            report.write(format!("\n</details>\n"));
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
                // /!\ (☞ﾟヮﾟ)☞ we remove native frames (FctId == 0) and keep only managed ones
                new_key.retain(|&x| x != 0);
                // reverse the stack
                new_key.reverse();
                sequences.insert(new_key, value);
            }
        }
        
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        
        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len()));

        let nb_roots = tree.children.len();
        let nb_threads: usize = tree.children.iter().map(|x| x.get_inclusive_value().0.len()).sum();
        
        let mut report = self.session_info.create_report("summary.md".to_owned());
        report.write_line(format!("# Aggregated Callstacks"));
        report.write_line(format!("\n==> {nb_threads} threads with {nb_roots} roots"));

        let mut report_html = self.session_info.create_report("summary.html".to_owned());
        report_html.write_line(format!("<h3>{nb_threads} threads <small class=\"text-muted\">with {nb_roots} roots</small></h3>"));

        for tree_node in tree.children {
            report.write(format!("\n{}", str::repeat("_", 50)));
            self.print(&tree_node, 0, &mut report);
            self.print_html(&tree_node, false, &mut report_html);
        }

        self.clr().request_profiler_detach(3000)
    }
}

impl CorProfilerCallback4 for ParallelStacksProfiler {}
impl CorProfilerCallback5 for ParallelStacksProfiler {}
impl CorProfilerCallback6 for ParallelStacksProfiler {}
impl CorProfilerCallback7 for ParallelStacksProfiler {}
impl CorProfilerCallback8 for ParallelStacksProfiler {}
impl CorProfilerCallback9 for ParallelStacksProfiler {}