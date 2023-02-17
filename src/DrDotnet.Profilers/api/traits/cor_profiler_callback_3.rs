#![allow(unused_variables)]
use crate::{ffi::HRESULT, ClrProfilerInfo};
use std::ffi::c_void;

pub trait CorProfilerCallback3 {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
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
