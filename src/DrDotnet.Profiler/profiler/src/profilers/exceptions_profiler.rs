use dashmap::DashMap;
use profiling_api::*;
use uuid::Uuid;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::report::*;
use super::Profiler;
use super::ProfilerData;

pub struct ExceptionsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    exceptions: DashMap<String, AtomicUsize>,
}

impl ExceptionsProfiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for ExceptionsProfiler {
    fn clone(&self) -> Self { 
        ExceptionsProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            exceptions: DashMap::new()
        }
     }
}

impl ClrProfiler for ExceptionsProfiler {
    fn new() -> ExceptionsProfiler {
        ExceptionsProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            exceptions: DashMap::new()
        }
    }
}

impl Profiler for ExceptionsProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E82").unwrap(),
            name: "Exceptions Profiler".to_owned(),
            description: "Lists occuring exceptions by importance.\nHandled exceptions are also listed.".to_owned(),
        }
    }
}

impl CorProfilerCallback for ExceptionsProfiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), ffi::HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("[profiler] Initialize at start");

        // Set the event mask
        self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT> {

        let key = "my exception".to_owned();
        match self.exceptions.get_mut(&key) {
            Some(pair) => { pair.value().fetch_add(1, Ordering::Relaxed); },
            None => { self.exceptions.insert(key, AtomicUsize::new(1)); },
        }
        
        Ok(())
    }
}

impl CorProfilerCallback2 for ExceptionsProfiler {}

impl CorProfilerCallback3 for ExceptionsProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {

        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("[profiler] Initialize with attach");

        // Set the event mask
        //self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;
        self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS)?;

        unsafe {
            //let vec: [u8; 16] = [0; 16];
            //std::ptr::copy_nonoverlapping(vec.as_ptr(), client_data as *mut u8, vec.len());
            //self.session_id = Uuid::from_bytes(*voidp_to_ref::<[u8; 16]>(client_data));
            let cstr = std::ffi::CStr::from_ptr(client_data as *const _).to_string_lossy();
            self.session_id = Uuid::parse_str(&cstr).unwrap();
        }

        println!("[profiler] Session uuid: {:?}", self.session_id);

        Ok(())
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {

        println!("[profiler] Profiler successfully attached!");

        use std::thread::*;

        let pi = self.profiler_info().clone();

        thread::spawn(move || {
            sleep(Duration::from_secs(10));
            // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo3-requestprofilerdetach-method
            // https://github.com/Potapy4/dotnet-coreclr/blob/master/Documentation/Profiling/davbr-blog-archive/Profiler%20Detach.md#requestprofilerdetach
            pi.request_profiler_detach(3000);
        });

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {

        println!("[profiler] Profiler successfully detached!");

        let session = Session::create_session(self.session_id, ExceptionsProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Exceptions Report"));
        report.write_line(format!("## Exceptions by Occurrences"));

        for exception in &self.exceptions {
            report.write_line(format!("- {}: {}", exception.key(), exception.value().load(Ordering::Relaxed)));
        }

        println!("[profiler] Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for ExceptionsProfiler {}
impl CorProfilerCallback5 for ExceptionsProfiler {}
impl CorProfilerCallback6 for ExceptionsProfiler {}
impl CorProfilerCallback7 for ExceptionsProfiler {}
impl CorProfilerCallback8 for ExceptionsProfiler {}
impl CorProfilerCallback9 for ExceptionsProfiler {}

#[derive(Clone)]
struct MyStruct();

impl MyStruct{

    fn fire_and_forget(&self) {
        let slf = self.clone();
        thread::spawn(move || {
            slf.do_something();
        });
    }

    fn do_something(&self) {
        println!("Hello!");
    }
}