#![allow(non_snake_case)]
use crate::ffi::{
    ICorProfilerFunctionControl, IUnknown, COR_IL_MAP, DWORD, HRESULT, LPCBYTE, ULONG,
};

#[repr(C)]
pub struct CorProfilerFunctionControlVtbl {
    pub IUnknown: IUnknown<CorProfilerFunctionControl>,
    pub ICorProfilerFunctionControl: ICorProfilerFunctionControl<CorProfilerFunctionControl>,
}

#[repr(C)]
pub struct CorProfilerFunctionControl {
    pub lpVtbl: *const CorProfilerFunctionControlVtbl,
}

impl CorProfilerFunctionControl {
    pub unsafe fn i_cor_profiler_function_control(&self) -> &ICorProfilerFunctionControl<Self> {
        &(*self.lpVtbl).ICorProfilerFunctionControl
    }
    pub unsafe fn SetCodegenFlags(&self, flags: DWORD) -> HRESULT {
        (self.i_cor_profiler_function_control().SetCodegenFlags)(self, flags)
    }
    pub unsafe fn SetILFunctionBody(
        &self,
        cbNewILMethodHeader: ULONG,
        pbNewILMethodHeader: LPCBYTE,
    ) -> HRESULT {
        (self.i_cor_profiler_function_control().SetILFunctionBody)(
            self,
            cbNewILMethodHeader,
            pbNewILMethodHeader,
        )
    }
    pub unsafe fn SetILInstrumentedCodeMap(
        &self,
        cILMapEntries: ULONG,
        rgILMapEntries: *const COR_IL_MAP,
    ) -> HRESULT {
        (self
            .i_cor_profiler_function_control()
            .SetILInstrumentedCodeMap)(self, cILMapEntries, rgILMapEntries)
    }
}
