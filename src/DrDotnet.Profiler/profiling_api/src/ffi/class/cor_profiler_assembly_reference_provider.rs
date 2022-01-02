#![allow(non_snake_case)]
use crate::ffi::{
    ICorProfilerAssemblyReferenceProvider, IUnknown, COR_PRF_ASSEMBLY_REFERENCE_INFO, HRESULT,
};

#[repr(C)]
pub struct CorProfilerAssemblyReferenceProviderVtbl {
    pub IUnknown: IUnknown<CorProfilerAssemblyReferenceProvider>,
    pub ICorProfilerAssemblyReferenceProvider:
        ICorProfilerAssemblyReferenceProvider<CorProfilerAssemblyReferenceProvider>,
}

#[repr(C)]
pub struct CorProfilerAssemblyReferenceProvider {
    pub lpVtbl: *const CorProfilerAssemblyReferenceProviderVtbl,
}

impl CorProfilerAssemblyReferenceProvider {
    unsafe fn i_cor_profiler_assembly_reference_provider(
        &self,
    ) -> &ICorProfilerAssemblyReferenceProvider<Self> {
        &(*self.lpVtbl).ICorProfilerAssemblyReferenceProvider
    }
    pub unsafe fn AddAssemblyReference(
        &self,
        pAssemblyRefInfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO,
    ) -> HRESULT {
        (self
            .i_cor_profiler_assembly_reference_provider()
            .AddAssemblyReference)(self, pAssemblyRefInfo)
    }
}
