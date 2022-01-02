use profiling_api::*;
use profiling_api::ffi::{CorOpenFlags, FunctionID, COR_PRF_MONITOR, E_FAIL, HRESULT, ObjectID};
use profiling_api::cil::{nop, Method};

use std::slice;
use uuid::Uuid;

#[derive(Clone)]
pub struct ExceptionsProfiler {
    profiler_info: Option<ProfilerInfo>,
}

impl ExceptionsProfiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl ClrProfiler for ExceptionsProfiler {

    fn new() -> ExceptionsProfiler {
        ExceptionsProfiler {
            profiler_info: None,
        }
    }

    fn get_guid() -> Uuid { Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E82").unwrap() }
    fn get_name() -> String { "Exceptions Profiler".to_owned() }
    fn get_description() -> String { "Lists occuring exceptions by importance".to_owned() }
}

impl CorProfilerCallback for ExceptionsProfiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("initialize at start");

        // Set the event mask
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ObjectID) -> Result<(), HRESULT> {
        println!("exception_thrown");
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
    ) -> Result<(), HRESULT> {

        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        println!("initialize with attach");

        // Set the event mask
        self.profiler_info().set_event_mask(COR_PRF_MONITOR::COR_PRF_ALLOWABLE_AFTER_ATTACH)?;

        Ok(())
    }
}

impl CorProfilerCallback4 for ExceptionsProfiler {}
impl CorProfilerCallback5 for ExceptionsProfiler {}
impl CorProfilerCallback6 for ExceptionsProfiler {}
impl CorProfilerCallback7 for ExceptionsProfiler {}
impl CorProfilerCallback8 for ExceptionsProfiler {}
impl CorProfilerCallback9 for ExceptionsProfiler {}