#![allow(non_snake_case)]
use crate::ffi::{ModuleID, BYTE, DWORD, GUID, HRESULT};

#[repr(C)]
pub struct ICorProfilerInfo7<T> {
    pub ApplyMetaData: unsafe extern "system" fn(this: &T, moduleId: ModuleID) -> HRESULT,
    pub GetInMemorySymbolsLength: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        pCountSymbolBytes: *mut DWORD,
    ) -> HRESULT,
    pub ReadInMemorySymbols: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        symbolsReadOffset: DWORD,
        pSymbolBytes: *mut BYTE,
        countSymbolBytes: DWORD,
        pCountSymbolBytesRead: *mut DWORD,
    ) -> HRESULT,
}

impl ICorProfilerInfo7<()> {
    // 9AEECC0D-63E0-4187-8C00-E312F503F663
    pub const IID: GUID = GUID {
        data1: 0x9AEECC0D,
        data2: 0x63E0,
        data3: 0x4187,
        data4: [0x8C, 0x00, 0xE3, 0x12, 0xF5, 0x03, 0xF6, 0x63],
    };
}
