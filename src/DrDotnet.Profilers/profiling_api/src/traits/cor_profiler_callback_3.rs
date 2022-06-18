#![allow(unused_variables)]
use crate::{ffi::HRESULT, CorProfilerCallback2, ProfilerInfo};
use std::ffi::c_void;

pub trait CorProfilerCallback3: CorProfilerCallback2 {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ProfilerInfo,
        client_data: *const c_void,
        client_data_length: u32,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn profiler_attach_complete(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }
}
