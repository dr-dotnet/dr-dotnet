use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ Ordering, AtomicBool, AtomicIsize };
use itertools::Itertools;

use crate::api::*;
use crate::api::ffi::{FunctionID, HRESULT, ThreadID};
use crate::macros::*;
use crate::profilers::*;

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

#[derive(Default, Clone)]
struct StackFrame {
    // display: Option<str>,
    kind: StackFrameType,
    fct_id: FunctionID,
}

#[derive(Default, Clone)]
struct MergedStack {
    thread_ids: Vec<ThreadID>,
    stacks: Vec<MergedStack>,
    frame: StackFrame
}

impl MergedStack {
    
    pub fn push_thread_id(&mut self, thread_id: ThreadID) {
        self.thread_ids.push(thread_id);
    }
    
    pub fn add_stack(&mut self, thread_id: ThreadID, stack_trace: Vec<StackFrame>, index: Option<usize>) {
        self.thread_ids.push(thread_id);
        
        let mut index = index.unwrap_or(0);

        let mut first_frame: Option<&StackFrame> = None;
        
        for frame in &stack_trace[index..] {
            if frame.kind == StackFrameType::Managed { 
                first_frame = Some(frame);
                break
            }
            index += 1;
        }
        
        if first_frame.is_none() {
            return; 
        }
        let first_frame = first_frame.unwrap();
        
        let merged_stack = self.stacks.iter()
            .find_or_first(|s| s.frame.fct_id == first_frame.fct_id);
        
        if merged_stack.is_none() {
            let mut new_merged_stack = MergedStack {
                thread_ids: Vec::<ThreadID>::new(),
                stacks: Vec::<MergedStack>::new(),
                frame: first_frame.clone(),
            };
            if index == stack_trace.len() -1 {
                new_merged_stack.push_thread_id(thread_id);
            } else {
                new_merged_stack.add_stack(thread_id, stack_trace, Some(index + 1));
            }
            self.stacks.push(new_merged_stack);
            return;
        }
        
        let mut merged_stack = merged_stack.unwrap().clone();
        if index == stack_trace.len() -1 {
            merged_stack.push_thread_id(thread_id);
        } else {
            merged_stack.add_stack(thread_id, stack_trace, Some(index + 1));
        }
    }
}

impl Profiler for MergedCallStacksProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-A485B2056E71".to_owned(),
            name: "Merged call stacks Profiler".to_owned(),
            description: "Display a view of threads merged call stacks. ".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }
}

impl MergedCallStacksProfiler {

    fn print_callstacks(profiler_info: ClrProfilerInfo, mut merged_stack: std::sync::MutexGuard<MergedStack>)
    {
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
                    // display: None
                };
                stack_trace.push(frame);
            }
            
            if stack_trace.is_empty() { continue }
            
            merged_stack.add_stack(managed_thread_id, stack_trace, None);
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

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(40));

            // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
            if profiler_info.suspend_runtime().is_ok()
            {
                let k = merged_stack.lock().unwrap();
                MergedCallStacksProfiler::print_callstacks(profiler_info.clone(), k);
                if profiler_info.resume_runtime().is_err()
                {
                    error!("Can't resume runtime!");
                }
            }
            else
            {
                error!("Can't suspend runtime!");
            }
        });
        
        let session_info = self.session_info.clone();
        let clr = self.clr().clone();
        let merged_stack = self.merged_stack.clone();

        let callback = Box::new(move || {

            let mut report = session_info.create_report("summary.md".to_owned());
    
            report.write_line(format!("# Merged Callstacks"));
    
            let clr = clr.clone();
    
            let merged_stack = merged_stack.lock().unwrap();
    
            // for method in merged_stack.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            //     let method_id = *method.key();
            //     let name = match method_id {
            //         0 => "unmanaged".to_owned(),
            //         _ =>  unsafe { clr.get_full_method_name(*method.key()) }
            //     };
            //     report.write_line(format!("- {}: {}", name, method.value().load(Ordering::Relaxed)));
            // }
    
            info!("Report written");
        });

        detach_after_duration::<MergedCallStacksProfiler>(&self, 10, Some(callback));

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        Ok(())
    }
}

impl CorProfilerCallback4 for MergedCallStacksProfiler {}
impl CorProfilerCallback5 for MergedCallStacksProfiler {}
impl CorProfilerCallback6 for MergedCallStacksProfiler {}
impl CorProfilerCallback7 for MergedCallStacksProfiler {}
impl CorProfilerCallback8 for MergedCallStacksProfiler {}
impl CorProfilerCallback9 for MergedCallStacksProfiler {}