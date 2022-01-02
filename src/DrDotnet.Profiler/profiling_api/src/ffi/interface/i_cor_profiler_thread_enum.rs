#![allow(non_snake_case)]
use crate::ffi::{ThreadID, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerThreadEnum<T> {
    pub Skip: unsafe extern "system" fn(this: &T, celt: ULONG) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub Clone: unsafe extern "system" fn(this: &T, ppEnum: *mut *mut T) -> HRESULT,
    pub GetCount: unsafe extern "system" fn(this: &T, pcelt: *mut ULONG) -> HRESULT,
    pub Next: unsafe extern "system" fn(
        this: &T,
        celt: ULONG,
        ids: *mut ThreadID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}

impl ICorProfilerThreadEnum<()> {
    // 571194F7-25ED-419F-AA8B-7016B3159701
    pub const IID: GUID = GUID {
        data1: 0x571194F7,
        data2: 0x25ED,
        data3: 0x419F,
        data4: [0xAA, 0x8B, 0x70, 0x16, 0xB3, 0x15, 0x97, 0x01],
    };
}
