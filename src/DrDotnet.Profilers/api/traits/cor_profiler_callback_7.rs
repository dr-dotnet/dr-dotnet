#![allow(unused_variables)]
use crate::{
    ffi::{ModuleID, HRESULT},
};

pub trait CorProfilerCallback7 {
    fn module_in_memory_symbols_updated(&mut self, module_id: ModuleID) -> Result<(), HRESULT> {
        Ok(())
    }
}
