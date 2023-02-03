use dashmap::DashMap;
use uuid::Uuid;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ Ordering, AtomicBool, AtomicIsize };

use crate::api::*;
use crate::report::*;
use crate::profilers::*;

#[derive(Default)]
pub struct CpuHotpathProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    detached: Arc<AtomicBool>,
    calls: Arc<Mutex<DashMap<usize, AtomicIsize>>>,
}

impl Profiler for CpuHotpathProfiler {

    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-A485B2056E71").unwrap(),
            name: "CPU Hotpath Profiler".to_owned(),
            description: "Lists CPU hotpaths.".to_owned(),
            is_released: true, // If true, visible in release mode. Otherwise, profiler is only visible in debug mode
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl CpuHotpathProfiler {

    fn print_callstack(profiler_info: ProfilerInfo, calls: std::sync::MutexGuard<DashMap<usize, AtomicIsize>>)
    {
        let pinfo = profiler_info.clone();
        
        for managed_thread_id in pinfo.enum_threads().unwrap() {
            
            let method_ids = Vec::<ffi::FunctionID>::new();
            
            // We must pass this data as a pointer for callback to mutate it with actual method ids from stack walking
            let method_ids_ptr_c = &method_ids as *const Vec<ffi::FunctionID> as *mut std::ffi::c_void;

            _ = pinfo.do_stack_snapshot(managed_thread_id, crate::utils::stack_snapshot_callback, ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, method_ids_ptr_c, std::ptr::null(), 0);    

            for method_id in method_ids {
                match calls.get_mut(&method_id) {
                    Some(pair) => { pair.value().fetch_add(1, Ordering::Relaxed); },
                    None => { calls.insert(method_id, AtomicIsize::new(1)); },
                }
            }
        }
    }
}

impl CorProfilerCallback for CpuHotpathProfiler {}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        match init_session(client_data, client_data_length) {
            Ok(uuid) => {
                self.session_id = uuid;
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        let profiler_info = self.profiler_info().clone();

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
        let session_id = self.session_id.clone();
        let profiler_info = self.profiler_info().clone();
        let calls = self.calls.clone();

        let callback = Box::new(move || {

            // Mark profiler as detached as it is about to be detached
            // We want to do this before the profiler is fully detached
            detached.store(true, std::sync::atomic::Ordering::Relaxed);

            let session = Session::get_session(session_id, ExceptionsProfiler::get_info());

            let mut report = session.create_report("summary.md".to_owned());
    
            report.write_line(format!("# Method Calls"));
    
            use itertools::Itertools;
    
            let profiler_info = profiler_info.clone();
    
            let calls = calls.lock().unwrap();
    
            for method in calls.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
                let method_id = *method.key();
                let name = match method_id {
                    0 => "unmanaged".to_owned(),
                    _ =>  unsafe { extensions::get_full_method_name(&profiler_info, *method.key()) }
                };
                report.write_line(format!("- {}: {}", name, method.value().load(Ordering::Relaxed)));
            }
    
            info!("Report written");
        });

        detach_after_duration::<CpuHotpathProfiler>(&self, 10, Some(callback));

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}