#![allow(unused_variables)]
use crate::{
    ffi::{FunctionID, HRESULT, LPCBYTE},
    CorProfilerCallback7,
};

pub trait CorProfilerCallback8: CorProfilerCallback7 {
    fn dynamic_method_jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        is_safe_to_block: bool,
        il_header: LPCBYTE,
        il_header_length: u32,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn dynamic_method_jit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        f_is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }
}
