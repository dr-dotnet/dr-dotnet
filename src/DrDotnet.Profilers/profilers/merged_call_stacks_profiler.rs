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

const PADDING: usize = 5;
/// if < 0 will print all thread ids
const NB_THREAD_IDS_TO_PRINT: usize = 4;


#[derive(Default)]
pub struct MergedCallStacksProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    merged_stack: Arc<Mutex<MergedStack>>
}

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
enum StackFrameType {
    #[default]
    Managed,
    Native
}

#[derive(Default, Clone, Debug)]
struct StackFrame {
    display: Option<String>,
    kind: StackFrameType,
    fct_id: FunctionID,
}

#[derive(Default, Clone, Debug)]
struct MergedStack {
    thread_ids: Vec<ThreadID>,
    stacks: Vec<MergedStack>,
    frame: StackFrame
}

impl StackFrame {
    fn format(frame: &StackFrame, clr: &ClrProfilerInfo) -> String {
        match frame.kind {
            StackFrameType::Native => "unmanaged".to_string(),
            StackFrameType::Managed =>  unsafe { clr.get_full_method_name(frame.fct_id) }
        }
    }
}

impl MergedStack {
    
    pub fn push_thread_id(&mut self, thread_id: ThreadID) {
        self.thread_ids.push(thread_id);
    }
    
    pub fn add_stack(&mut self, clr: &ClrProfilerInfo, thread_id: ThreadID, stack_trace: Vec<StackFrame>, index: Option<usize>) {
        self.thread_ids.push(thread_id);
        
        let mut index = index.unwrap_or(0);
        let first_frame = &stack_trace[index];
        
        let mut merged_stack = self.stacks.iter_mut()
            .find(|s| s.frame.fct_id == first_frame.fct_id);
        
        if merged_stack.is_none() {
            self.stacks.push(MergedStack::new(&first_frame, clr));
            merged_stack = self.stacks.last_mut();
        }
        
        let merged_stack = merged_stack.unwrap();
        if index == stack_trace.len() -1 {
            merged_stack.push_thread_id(thread_id);
        } else {
            merged_stack.add_stack(clr, thread_id, stack_trace, Some(index + 1));
        }
    }
    
    pub fn new(frame: &StackFrame, clr: &ClrProfilerInfo) -> Self {
        MergedStack {
            thread_ids: Vec::new(),
            stacks: Vec::new(),
            frame: StackFrame {
                kind: frame.kind.clone(),
                fct_id: frame.fct_id,
                display: Some(StackFrame::format(frame, clr))
            }
        }
    }

    pub fn write_to(&self, report: &mut Report) {
        self.write_stack(report, 0);
    }

    fn write_stack(&self, report: &mut Report, increment: usize) {
        let alignment = str::repeat(" ", PADDING * increment);
        let new_line = format!("\r\n{alignment}");
        
        let thread_ids = format!(" ~~~~ {}", self.format_thread_ids());
        let thread_count = format!("{:>PADDING$} ", self.thread_ids.len());
        let frame = self.frame.display.as_ref().unwrap();
        
        if self.stacks.is_empty() {
            report.write(new_line.as_str());
            report.write(thread_ids);
            report.write(new_line.as_str());
            report.write(thread_count);
            report.write(frame);
            return;
        }
        
        for next_stack in self.stacks.iter()
            .sorted_by(|a, b| Ord::cmp(&b.thread_ids.len(), &a.thread_ids.len())) {
            let has_same_alignment = next_stack.thread_ids.len() == self.thread_ids.len();
            next_stack.write_stack(report, if has_same_alignment {increment} else { increment + 1 });
        }
        
        report.write(new_line.as_str());
        report.write(thread_count);
        report.write(frame);
    }
    
    fn format_thread_ids(&self) -> String {

        let count = self.thread_ids.len();
        let limit = min(count, NB_THREAD_IDS_TO_PRINT);

        if limit < 0 {
            return self.thread_ids.iter().map(|k| format!("{k}")).collect::<Vec<String>>().join(",");
        }
        
        let mut result = self.thread_ids.get(..limit).unwrap_or_default().iter().map(|k| format!("{k}")).collect::<Vec<String>>().join(",");
        if count > limit {
            result += "...";
        }
        result
    }
}

impl Profiler for MergedCallStacksProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "9404d16c-b49e-11ed-afa1-0242ac120002".to_owned(),
            name: "Merged call stacks Profiler".to_owned(),
            description: "Display a view of threads merged call stacks. ".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }
}

impl MergedCallStacksProfiler {

    fn build_callstacks(profiler_info: ClrProfilerInfo, mut merged_stack: std::sync::MutexGuard<MergedStack>)
    {
        info!("Starts building callstacks");
        let pinfo = profiler_info.clone();
        
        for managed_thread_id in pinfo.enum_threads().unwrap() {
            
            let method_ids = Vec::<ffi::FunctionID>::new();
            
            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let method_ids_ptr_c = &method_ids as *const Vec<ffi::FunctionID> as *mut std::ffi::c_void;
            
            let _ =  pinfo.do_stack_snapshot(
                managed_thread_id, 
                crate::utils::stack_snapshot_callback, 
                ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, 
                method_ids_ptr_c, 
                std::ptr::null(), 0);    

            let mut stack_trace = Vec::<StackFrame>::new();
            
            for method_id in method_ids.iter().rev() {
                let frame = StackFrame {
                    kind: if *method_id == 0 { StackFrameType::Native } else { StackFrameType::Managed },
                    fct_id: *method_id,
                    display: None
                };
                //TODO: handle correctly native frame, for now we just ignore them
                if frame.kind == StackFrameType::Native { continue } 
                stack_trace.push(frame);
            }
            
            if stack_trace.is_empty() { continue }
            
            merged_stack.add_stack(&profiler_info, managed_thread_id, stack_trace, None);
        }
    }
}

impl CorProfilerCallback for MergedCallStacksProfiler {}

impl CorProfilerCallback2 for MergedCallStacksProfiler {}

impl CorProfilerCallback3 for MergedCallStacksProfiler {
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        
        let profiler_info = self.clr().clone();
        let merged_stack = self.merged_stack.clone();

        let thread_handle = std::thread::spawn(move || {
            
            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            if profiler_info.suspend_runtime().is_ok() {
                
                let k = merged_stack.lock().unwrap();
                MergedCallStacksProfiler::build_callstacks(profiler_info.clone(), k);
                
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
        
        self.clr().request_profiler_detach(3000)
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        let mut report = self.session_info.create_report("summary.md".to_owned());
        report.write_line(format!("# Merged Callstacks"));
        
        let merged_stack = self.merged_stack.lock().unwrap();
        
        let nb_roots = merged_stack.stacks.len();
        let mut nb_threads = 0;
        
        for stack in merged_stack.stacks.iter() {
            nb_threads += stack.thread_ids.len();
            stack.write_to(&mut report);
            report.write(format!("\n\n{}", str::repeat("_", 50)));
        }
        
        report.write_line(format!("\n==> {} threads with {} roots", nb_threads, nb_roots));
        Ok(())
    }
}

impl CorProfilerCallback4 for MergedCallStacksProfiler {}
impl CorProfilerCallback5 for MergedCallStacksProfiler {}
impl CorProfilerCallback6 for MergedCallStacksProfiler {}
impl CorProfilerCallback7 for MergedCallStacksProfiler {}
impl CorProfilerCallback8 for MergedCallStacksProfiler {}
impl CorProfilerCallback9 for MergedCallStacksProfiler {}