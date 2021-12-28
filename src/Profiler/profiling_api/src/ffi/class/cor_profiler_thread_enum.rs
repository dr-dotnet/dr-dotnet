#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerThreadEnum, IUnknown, ThreadID, HRESULT, ULONG};

#[repr(C)]
pub struct CorProfilerThreadEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerThreadEnum>,
    pub ICorProfilerThreadEnum: ICorProfilerThreadEnum<CorProfilerThreadEnum>,
}

#[repr(C)]
pub struct CorProfilerThreadEnum {
    pub lpVtbl: *const CorProfilerThreadEnumVtbl,
}

impl CorProfilerThreadEnum {
    pub unsafe fn i_cor_profiler_thread_enum(&self) -> &ICorProfilerThreadEnum<Self> {
        &(*self.lpVtbl).ICorProfilerThreadEnum
    }
    pub unsafe fn Skip(&self, celt: ULONG) -> HRESULT {
        (self.i_cor_profiler_thread_enum().Skip)(self, celt)
    }
    pub unsafe fn Reset(&self) -> HRESULT {
        (self.i_cor_profiler_thread_enum().Reset)(self)
    }
    pub unsafe fn Clone(&self, ppEnum: *mut *mut Self) -> HRESULT {
        (self.i_cor_profiler_thread_enum().Clone)(self, ppEnum)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_thread_enum().GetCount)(self, pcelt)
    }
    pub unsafe fn Next(
        &self,
        celt: ULONG,
        ids: *mut ThreadID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_thread_enum().Next)(self, celt, ids, pceltFetched)
    }
}
