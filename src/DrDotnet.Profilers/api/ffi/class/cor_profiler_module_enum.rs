#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerModuleEnum, IUnknown, ModuleID, HRESULT, ULONG};

#[repr(C)]
pub struct CorProfilerModuleEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerModuleEnum>,
    pub ICorProfilerModuleEnum: ICorProfilerModuleEnum<CorProfilerModuleEnum>,
}

#[repr(C)]
pub struct CorProfilerModuleEnum {
    pub lpVtbl: *const CorProfilerModuleEnumVtbl,
}

impl CorProfilerModuleEnum {
    pub unsafe fn i_cor_profiler_module_enum(&self) -> &ICorProfilerModuleEnum<Self> {
        &(*self.lpVtbl).ICorProfilerModuleEnum
    }
    pub unsafe fn Skip(&self, celt: ULONG) -> HRESULT {
        (self.i_cor_profiler_module_enum().Skip)(self, celt)
    }
    pub unsafe fn Reset(&self) -> HRESULT {
        (self.i_cor_profiler_module_enum().Reset)(self)
    }
    pub unsafe fn Clone(&self, ppEnum: *mut *mut Self) -> HRESULT {
        (self.i_cor_profiler_module_enum().Clone)(self, ppEnum)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_module_enum().GetCount)(self, pcelt)
    }
    pub unsafe fn Next(
        &self,
        celt: ULONG,
        objects: *mut ModuleID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_module_enum().Next)(self, celt, objects, pceltFetched)
    }
}
