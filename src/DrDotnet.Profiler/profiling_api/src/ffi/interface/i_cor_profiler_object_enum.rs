#![allow(non_snake_case)]
use crate::ffi::{ObjectID, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerObjectEnum<T> {
    pub Skip: unsafe extern "system" fn(this: &T, celt: ULONG) -> HRESULT,
    pub Reset: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub Clone: unsafe extern "system" fn(this: &T, ppEnum: *mut *mut T) -> HRESULT,
    pub GetCount: unsafe extern "system" fn(this: &T, pcelt: *mut ULONG) -> HRESULT,
    pub Next: unsafe extern "system" fn(
        this: &T,
        celt: ULONG,
        objects: *mut ObjectID,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
}

impl ICorProfilerObjectEnum<()> {
    // 2C6269BD-2D13-4321-AE12-6686365FD6AF
    pub const IID: GUID = GUID {
        data1: 0x2C6269BD,
        data2: 0x2D13,
        data3: 0x4321,
        data4: [0xAE, 0x12, 0x66, 0x86, 0x36, 0x5F, 0xD6, 0xAF],
    };
}
