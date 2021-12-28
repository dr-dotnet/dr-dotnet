#![allow(non_snake_case)]
use crate::ffi::{ModuleID, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerModuleEnum<T> {
    pub Skip: unsafe extern "system" fn(this: &T, celt: ULONG) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub Clone: unsafe extern "system" fn(this: &T, ppEnum: *mut *mut T) -> HRESULT,
    pub GetCount: unsafe extern "system" fn(this: &T, pcelt: *mut ULONG) -> HRESULT,
    pub Next: unsafe extern "system" fn(
        this: &T,
        celt: ULONG,
        objects: *mut ModuleID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}

impl ICorProfilerModuleEnum<()> {
    // B0266D75-2081-4493-AF7F-028BA34DB891
    pub const IID: GUID = GUID {
        data1: 0xB0266D75,
        data2: 0x2081,
        data3: 0x4493,
        data4: [0xAF, 0x7F, 0x02, 0x8B, 0xA3, 0x4D, 0xB8, 0x91],
    };
}
