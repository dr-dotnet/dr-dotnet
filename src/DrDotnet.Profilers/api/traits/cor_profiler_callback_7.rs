#![allow(unused_variables)]
use crate::{
    ffi::{ModuleID, HRESULT},
    CorProfilerCallback6,
};

pub trait CorProfilerCallback7: CorProfilerCallback6 {
    fn module_in_memory_symbols_updated(&mut self, module_id: ModuleID) -> Result<(), HRESULT> {
        Ok(())
    }
}
