#![allow(non_snake_case)]
use crate::ffi::{COR_PRF_METHOD, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerMethodEnum<T> {
    pub Skip: unsafe extern "system" fn(this: &T, celt: ULONG) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub Clone: unsafe extern "system" fn(this: &T, ppEnum: *mut *mut T) -> HRESULT,
    pub GetCount: unsafe extern "system" fn(this: &T, pcelt: *mut ULONG) -> HRESULT,
    pub Next: unsafe extern "system" fn(
        this: &T,
        celt: ULONG,
        elements: *mut COR_PRF_METHOD,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}

impl ICorProfilerMethodEnum<()> {
    // FCCEE788-0088-454B-A811-C99F298D1942
    pub const IID: GUID = GUID {
        data1: 0xFCCEE788,
        data2: 0x0088,
        data3: 0x454B,
        data4: [0xA8, 0x11, 0xC9, 0x9F, 0x29, 0x8D, 0x19, 0x42],
    };
}
