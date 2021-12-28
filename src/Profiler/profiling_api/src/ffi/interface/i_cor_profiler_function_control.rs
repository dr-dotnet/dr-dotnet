#![allow(non_snake_case)]
use crate::ffi::{COR_IL_MAP, DWORD, GUID, HRESULT, LPCBYTE, ULONG};

#[repr(C)]
pub struct ICorProfilerFunctionControl<T> {
    pub SetCodegenFlags: unsafe extern "system" fn(this: &T, flags: DWORD) -> HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(
        this: &T,
        cbNewILMethodHeader: ULONG,
        pbNewILMethodHeader: LPCBYTE,
    ) -> HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(
        this: &T,
        cILMapEntries: ULONG,
        rgILMapEntries: *const COR_IL_MAP,
    ) -> HRESULT,
}

impl ICorProfilerFunctionControl<()> {
    // F0963021-E1EA-4732-8581-E01B0BD3C0C6
    pub const IID: GUID = GUID {
        data1: 0xF0963021,
        data2: 0xE1EA,
        data3: 0x4732,
        data4: [0x85, 0x81, 0xE0, 0x1B, 0x0B, 0xD3, 0xC0, 0xC6],
    };
}
