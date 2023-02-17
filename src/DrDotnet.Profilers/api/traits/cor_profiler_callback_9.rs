#![allow(unused_variables)]
use crate::{
    ffi::{FunctionID, HRESULT},
};

pub trait CorProfilerCallback9 {
    fn dynamic_method_unloaded(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }
}
