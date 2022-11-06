use profiling_api::*;
use uuid::Uuid;
use std::sync::Arc;
use std::sync::atomic::{ Ordering, AtomicBool };

use crate::report::*;
use crate::profilers::*;

#[derive(Default)]
pub struct CpuHotpathProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    detached: Arc<AtomicBool>,
}

impl Profiler for CpuHotpathProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-A485B2056E71").unwrap(),
            name: "CPU Hotpath Profiler".to_owned(),
            description: "Lists CPU hotpaths.".to_owned(),
            is_released: true,
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl CpuHotpathProfiler {

    fn print_callstack(profiler_info: ProfilerInfo)
    {
        let pinfo = profiler_info.clone();
        
        for managed_thread_id in pinfo.enum_threads().unwrap() {
            
            let mut v = Vec::<usize>::new();
            
            let vecptr_c = &v as *const Vec<usize> as *mut std::ffi::c_void;

            let res = pinfo.do_stack_snapshot(managed_thread_id, crate::utils::stack_snapshot_callback, ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, vecptr_c, std::ptr::null(), 0);    
        
            warn!("--- Thread ID: {} ---", managed_thread_id);

            for method_id in v {
                let name = unsafe { extensions::get_method_name(&pinfo, method_id) };
                warn!("- {}", name);
            }
        }
    }
}

impl CorProfilerCallback for CpuHotpathProfiler
{
    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT>
    {
        info!("exception thrown");
        
        CpuHotpathProfiler::print_callstack(self.profiler_info().clone());

        Ok(())
    }
}

impl CorProfilerCallback2 for CpuHotpathProfiler {}

impl CorProfilerCallback3 for CpuHotpathProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS | ffi::COR_PRF_MONITOR::COR_PRF_ENABLE_STACK_SNAPSHOT) {
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

        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(400));

                if detached.load(Ordering::Relaxed) {
                    warn!("detached");
                    break;
                }

                // https://github.com/dotnet/runtime/issues/37586#issuecomment-641114483
                if profiler_info.suspend_runtime().is_ok()
                {
                    CpuHotpathProfiler::print_callstack(profiler_info.clone());
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

        let callback = Box::new(move || {
            detached.store(true, std::sync::atomic::Ordering::Relaxed);
        });

        detach_after_duration::<CpuHotpathProfiler>(&self, 10, Some(callback));
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        // todo: Do this in detach timer? Seems to be too late here

        let session = Session::get_session(self.session_id, ExceptionsProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Hello"));

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for CpuHotpathProfiler {}
impl CorProfilerCallback5 for CpuHotpathProfiler {}
impl CorProfilerCallback6 for CpuHotpathProfiler {}
impl CorProfilerCallback7 for CpuHotpathProfiler {}
impl CorProfilerCallback8 for CpuHotpathProfiler {}
impl CorProfilerCallback9 for CpuHotpathProfiler {}