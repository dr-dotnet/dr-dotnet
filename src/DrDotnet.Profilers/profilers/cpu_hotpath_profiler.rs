use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ Ordering, AtomicBool, AtomicIsize };

use crate::api::*;
use crate::api::ffi::{HRESULT, HResult};
use crate::macros::*;
use crate::profilers::*;

#[derive(Default)]
pub struct CpuHotpathProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    detached: Arc<AtomicBool>,
    calls: Arc<Mutex<DashMap<usize, AtomicIsize>>>,
}

impl Profiler for CpuHotpathProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-A485B2056E71".to_owned(),
            name: "CPU Hotpath Profiler".to_owned(),
            description: "Lists CPU hotpaths.".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }
}

impl CpuHotpathProfiler {

    fn print_callstack(profiler_info: ClrProfilerInfo, calls: std::sync::MutexGuard<DashMap<usize, AtomicIsize>>)
    {
        let pinfo = profiler_info.clone();

        // Variables to debug do_stack_snapshot calls
        let mut nbThreads = 0;
        let mut errors: HashMap<HRESULT, i32> = HashMap::new();
        
        for managed_thread_id in pinfo.enum_threads().unwrap() {
            nbThreads += 1;
            let method_ids = Vec::<ffi::FunctionID>::new();
            
            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let method_ids_ptr_c = &method_ids as *const Vec<ffi::FunctionID> as *mut std::ffi::c_void;
            
            match pinfo.do_stack_snapshot(
                managed_thread_id, 
                crate::utils::stack_snapshot_callback, 
                ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, 
                method_ids_ptr_c, 
                std::ptr::null(), 0){
                Ok(_) => {}
                Err(e) => {
                    match errors.get(&e) {
                        Some(nb) => { errors.insert(e, nb + 1);},
                        None => { errors.insert(e, 1); }
                    }
                }
            };    

            for method_id in method_ids {
                match calls.get_mut(&method_id) {
                    Some(pair) => { pair.value().fetch_add(1, Ordering::Relaxed); },
                    None => { calls.insert(method_id, AtomicIsize::new(1)); },
                }
            }
        }
        
        let nb_errors: i32 = errors.values().sum();
        debug!("Nb threads: {nbThreads}, do_stack_snapshot failed {nb_errors} time(s) with error(s): {}",
            errors.iter().map(|(k, v)| format!("{}:{v}", HResult {value:*k})).collect::<Vec<String>>().join(","));

    }
}

impl CorProfilerCallback for CpuHotpathProfiler {}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler {
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        let profiler_info = self.clr().clone();

        self.detached.store(false, std::sync::atomic::Ordering::Relaxed);

        let detached = self.detached.clone();

        let calls = self.calls.clone();

        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(40));

                // Profile until profiler detach has been requested
                if detached.load(Ordering::Relaxed) {
                    break;
                }

                // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
                if profiler_info.suspend_runtime().is_ok()
                {
                    let k = calls.lock().unwrap();
                    CpuHotpathProfiler::print_callstack(profiler_info.clone(), k);
                    if profiler_info.resume_runtime().is_err()
                    {
                        error!("Can't resume runtime!");
                    }
                }
                else
                {
                    error!("Can't suspend runtime!");
                }
            }
        });

        let detached = self.detached.clone();
        let session_info = self.session_info.clone();
        let clr = self.clr().clone();
        let calls = self.calls.clone();

        let callback = Box::new(move || {

            // Mark profiler as detached as it is about to be detached
            // We want to do this before the profiler is fully detached
            detached.store(true, std::sync::atomic::Ordering::Relaxed);

            let mut report = session_info.create_report("summary.md".to_owned());
    
            report.write_line(format!("# Method Calls"));
    
            use itertools::Itertools;
    
            let clr = clr.clone();
    
            let calls = calls.lock().unwrap();
    
            for method in calls.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
                let method_id = *method.key();
                let name = match method_id {
                    0 => "unmanaged".to_owned(),
                    _ =>  unsafe { clr.get_full_method_name(*method.key()) }
                };
                report.write_line(format!("- {}: {}", name, method.value().load(Ordering::Relaxed)));
            }
    
            info!("Report written");
        });

        detach_after_duration::<CpuHotpathProfiler>(&self, 10, Some(callback));

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}