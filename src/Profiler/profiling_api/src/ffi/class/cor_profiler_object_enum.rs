#![allow(non_snake_case)]
use crate::ffi::{ICorProfilerObjectEnum, IUnknown, ObjectID, HRESULT, ULONG};

#[repr(C)]
pub struct CorProfilerObjectEnumVtbl {
    pub IUnknown: IUnknown<CorProfilerObjectEnum>,
    pub ICorProfilerObjectEnum: ICorProfilerObjectEnum<CorProfilerObjectEnum>,
}

#[repr(C)]
pub struct CorProfilerObjectEnum {
    pub lpVtbl: *const CorProfilerObjectEnumVtbl,
}

impl CorProfilerObjectEnum {
    pub unsafe fn i_cor_profiler_object_enum(&self) -> &ICorProfilerObjectEnum<Self> {
        &(*self.lpVtbl).ICorProfilerObjectEnum
    }
    pub unsafe fn Skip(&self, celt: ULONG) -> HRESULT {
        (self.i_cor_profiler_object_enum().Skip)(self, celt)
    }
    pub unsafe fn Reset(&self) -> HRESULT {
        (self.i_cor_profiler_object_enum().Reset)(self)
    }
    pub unsafe fn Clone(&self, ppEnum: *mut *mut Self) -> HRESULT {
        (self.i_cor_profiler_object_enum().Clone)(self, ppEnum)
    }
    pub unsafe fn GetCount(&self, pcelt: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_object_enum().GetCount)(self, pcelt)
    }
    pub unsafe fn Next(
        &self,
        celt: ULONG,
        objects: *mut ObjectID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_object_enum().Next)(self, celt, objects, pceltFetched)
    }
}
