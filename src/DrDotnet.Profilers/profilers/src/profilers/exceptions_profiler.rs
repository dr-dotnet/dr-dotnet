use dashmap::DashMap;
use profiling_api::*;
use uuid::Uuid;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::report::*;
use crate::profilers::*;

pub struct ExceptionsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    exceptions: DashMap<String, AtomicIsize>,
}

impl Profiler for ExceptionsProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E82").unwrap(),
            name: "Exceptions Profiler".to_owned(),
            description: "Lists occuring exceptions by importance.\nHandled exceptions are also listed.".to_owned(),
            isReleased: true,
        }
    }

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

impl CorProfilerCallback for ExceptionsProfiler
{
    fn exception_thrown(&mut self, thrown_object_id: ffi::ObjectID) -> Result<(), ffi::HRESULT>
    {
        let pinfo = self.profiler_info();
        let name = 
        match pinfo.get_class_from_object(thrown_object_id) {
            Ok(class_id) =>
            match pinfo.get_class_id_info(class_id) {
                Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                _ => "unknown2".to_owned()
            },
            _ => "unknown1".to_owned()
        };

        let key = name;
        match self.exceptions.get_mut(&key) {
            Some(pair) => { pair.value().fetch_add(1, Ordering::Relaxed); },
            None => { self.exceptions.insert(key, AtomicIsize::new(1)); },
        }
        
        Ok(())
    }
}

impl CorProfilerCallback2 for ExceptionsProfiler {}

impl CorProfilerCallback3 for ExceptionsProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_EXCEPTIONS) {
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
        detach_after_duration::<ExceptionsProfiler>(&self, 10);
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

impl CorProfilerCallback4 for ExceptionsProfiler {}
impl CorProfilerCallback5 for ExceptionsProfiler {}
impl CorProfilerCallback6 for ExceptionsProfiler {}
impl CorProfilerCallback7 for ExceptionsProfiler {}
impl CorProfilerCallback8 for ExceptionsProfiler {}
impl CorProfilerCallback9 for ExceptionsProfiler {}