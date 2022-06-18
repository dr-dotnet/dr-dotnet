#![allow(non_snake_case)]
use crate::ffi::{GUID, HRESULT, REFIID, ULONG};
use std::ffi::c_void;

#[repr(C)]
pub struct IUnknown<T> {
    pub QueryInterface: unsafe extern "system" fn(
        this: &mut T,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: &mut T) -> ULONG,
    pub Release: unsafe extern "system" fn(this: &mut T) -> ULONG,
}

impl IUnknown<()> {
    // 00000000-0000-0000-C000-000000000046
    pub const IID: GUID = GUID {
        data1: 0x00000000,
        data2: 0x0000,
        data3: 0x0000,
        data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    };
}
