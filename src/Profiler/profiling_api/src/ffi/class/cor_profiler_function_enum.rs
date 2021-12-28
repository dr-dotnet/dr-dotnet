#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerFunctionEnum, IUnknown, COR_PRF_FUNCTION, HRESULT, ULONG};

#[repr(C)]
pub struct CorProfilerFunctionEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerFunctionEnum>,
    pub ICorProfilerFunctionEnum: ICorProfilerFunctionEnum<CorProfilerFunctionEnum>,
}

#[repr(C)]
pub struct CorProfilerFunctionEnum {
    pub lpVtbl: *const CorProfilerFunctionEnumVtbl,
}

impl CorProfilerFunctionEnum {
    pub unsafe fn i_cor_profiler_function_enum(&self) -> &ICorProfilerFunctionEnum<Self> {
        &(*self.lpVtbl).ICorProfilerFunctionEnum
    }
    pub unsafe fn Skip(&self, celt: ULONG) -> HRESULT {
        (self.i_cor_profiler_function_enum().Skip)(self, celt)
    }
    pub unsafe fn Reset(&self) -> HRESULT {
        (self.i_cor_profiler_function_enum().Reset)(self)
    }
    pub unsafe fn Clone(&self, ppEnum: *mut *mut Self) -> HRESULT {
        (self.i_cor_profiler_function_enum().Clone)(self, ppEnum)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_function_enum().GetCount)(self, pcelt)
    }
    pub unsafe fn Next(
        &self,
        celt: ULONG,
        ids: *mut COR_PRF_FUNCTION,
        pceltFetched: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_function_enum().Next)(self, celt, ids, pceltFetched)
    }
}
