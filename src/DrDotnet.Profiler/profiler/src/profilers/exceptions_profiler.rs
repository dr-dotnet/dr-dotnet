use profiling_api::*;
use profiling_api::ffi::{CorOpenFlags, FunctionID, COR_PRF_MONITOR, E_FAIL, HRESULT, ObjectID};
use profiling_api::cil::{nop, Method};

use std::slice;
use uuid::Uuid;

use std::fs::File;
use std::io::Write;
use crate::report::*;

use std::thread;
use std::time::Duration;

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ExceptionsProfiler {
    profiler_info: Option<ProfilerInfo>,
    exceptions_thrown: AtomicUsize,
    session_id: Uuid,
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
            exceptions_thrown: AtomicUsize::new(0),
            session_id: self.session_id.clone()
        }
     }
}

impl ClrProfiler for ExceptionsProfiler {
    fn new() -> ExceptionsProfiler {
        ExceptionsProfiler {
            profiler_info: None,
            exceptions_thrown: AtomicUsize::new(0),
            session_id: Uuid::default()
        }
    }
}

use super::Profiler;
use super::ProfilerData;

impl Profiler for ExceptionsProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            guid: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E82").unwrap(),
            name: "Exceptions Profiler".to_owned(),
            description: "Lists occuring exceptions by importance.\nHandled exceptions are also listed.".to_owned(),
        }
    }
}

impl CorProfilerCallback for ExceptionsProfiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("[profiler] Initialize at start");

        // Set the event mask
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ObjectID) -> Result<(), HRESULT> {
        self.exceptions_thrown.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl CorProfilerCallback2 for ExceptionsProfiler {}

unsafe fn voidp_to_ref<'a, T>(p: *const std::os::raw::c_void) -> &'a T
{
    unsafe { &*(p as *const T) }
}

impl CorProfilerCallback3 for ExceptionsProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), HRESULT> {

        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("[profiler] Initialize with attach");

        // Set the event mask
        //self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS)?;

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

    fn profiler_attach_complete(&mut self) -> Result<(), HRESULT> {

        println!("[profiler] Profiler successfully attached!");

        let result = self.profiler_info().request_profiler_detach(10000);
        println!("[profiler] Detach request result: {:?}", result);

        //thread::spawn(|| {
        //  thread::sleep(Duration::from_millis(5000));
        //  self.profiler_info().request_profiler_detach(10);
        //});

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), HRESULT> {

        println!("[profiler] Profiler successfully detached!");

        let mut entry = ReportEntry {
            name: "Exceptions".to_owned(),
            content: format!("exceptions:{}", self.exceptions_thrown.load(Ordering::Relaxed))
        };

        let mut section = ReportSection {
            name: "Exceptions".to_owned(),
            entries: vec![entry]
        };

        let mut report = Report{
            guid: uuid::Uuid::default(),
            name: String::default(),
            timestamp: chrono::offset::Local::now(),
            profiler: ExceptionsProfiler::get_info(),
            sections: vec![section]
        };
    
        report.name = "My Exceptions Report".to_owned();
    
        let json = serde_json::to_string_pretty(&report).unwrap();
        std::fs::create_dir_all("/dr-dotnet");
    
        let mut f = File::create(format!("/dr-dotnet/{}.json", self.session_id)).expect("Unable to create file");
        f.write_all(json.as_bytes()).expect("Unable to write data");    

        println!("[profiler] Report written to {}/tmp/report.json", std::env::current_dir().unwrap().display());

        Ok(())
    }
}

impl CorProfilerCallback4 for ExceptionsProfiler {}
impl CorProfilerCallback5 for ExceptionsProfiler {}
impl CorProfilerCallback6 for ExceptionsProfiler {}
impl CorProfilerCallback7 for ExceptionsProfiler {}
impl CorProfilerCallback8 for ExceptionsProfiler {}
impl CorProfilerCallback9 for ExceptionsProfiler {}