use dashmap::DashMap;
use profiling_api::*;
use uuid::Uuid;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::report::*;
use crate::profilers::*;

#[derive(Default)]
pub struct CpuHotpathProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    exceptions: DashMap<String, AtomicIsize>,
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

impl CorProfilerCallback for CpuHotpathProfiler
{
    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT>
    {
        info!("exception thrown");
        
        let pinfo = self.profiler_info();
        
        for managed_thread_id in pinfo.enum_threads().unwrap() {
               
            let mut v = Vec::<usize>::new();
            let dd = Box::new(v);

            {
                
                let pp = Box::into_raw(dd.clone());

                let state_ptr = pp as *mut std::ffi::c_void;

                //let state_ptr = v.as_ptr() as *mut std::ffi::c_void;
                
                let res = pinfo.do_stack_snapshot(managed_thread_id, crate::utils::stack_snapshot_callback2, ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, state_ptr, std::ptr::null(), 0);    
            
                error!("Caca: {:?}", res)
            }

            warn!("--- Thread ID: {} --- {}", managed_thread_id, dd.len());
            
            let t = *dd;
            
            for method_id in t {
                let name = unsafe { extensions::get_method_name(pinfo, method_id) };
                warn!("Thread ID: {}, Stacktrace: {}", managed_thread_id, name);
            }
        }

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
        let pinfo = self.profiler_info();



        detach_after_duration::<CpuHotpathProfiler>(&self, 10);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, ExceptionsProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Exceptions Report"));
        report.write_line(format!("## Exceptions by Occurrences"));

        use itertools::Itertools;

        for exception in self.exceptions.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            report.write_line(format!("- {}: {}", exception.key(), exception.value().load(Ordering::Relaxed)));
        }

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