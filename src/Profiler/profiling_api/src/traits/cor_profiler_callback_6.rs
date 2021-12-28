#![allow(unused_variables)]
use crate::{
    ffi::{CorProfilerAssemblyReferenceProvider, HRESULT},
    CorProfilerCallback5,
};

pub trait CorProfilerCallback6: CorProfilerCallback5 {
    fn get_assembly_references(
        &mut self,
        assembly_path: &str,
        asm_ref_provider: &CorProfilerAssemblyReferenceProvider,
    ) -> Result<(), HRESULT> {
        Ok(())
    }
}
