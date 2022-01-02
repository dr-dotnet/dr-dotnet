#![allow(unused_variables)]
use crate::{
    ffi::{FunctionID, HRESULT},
    CorProfilerCallback8,
};

pub trait CorProfilerCallback9: CorProfilerCallback8 {
    fn dynamic_method_unloaded(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }
}
