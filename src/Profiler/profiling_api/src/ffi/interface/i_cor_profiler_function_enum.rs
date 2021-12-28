#![allow(non_snake_case)]
use crate::ffi::{COR_PRF_FUNCTION, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerFunctionEnum<T> {
    pub Skip: unsafe extern "system" fn(this: &T, celt: ULONG) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub Clone: unsafe extern "system" fn(this: &T, ppEnum: *mut *mut T) -> HRESULT,
    pub GetCount: unsafe extern "system" fn(this: &T, pcelt: *mut ULONG) -> HRESULT,
    pub Next: unsafe extern "system" fn(
        this: &T,
        celt: ULONG,
        ids: *mut COR_PRF_FUNCTION,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}

impl ICorProfilerFunctionEnum<()> {
    // FF71301A-B994-429D-A10B-B345A65280EF
    pub const IID: GUID = GUID {
        data1: 0xFF71301A,
        data2: 0xB994,
        data3: 0x429D,
        data4: [0xA1, 0x0B, 0xB3, 0x45, 0xA6, 0x52, 0x80, 0xEF],
    };
}
