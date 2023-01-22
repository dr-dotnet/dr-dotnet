#![allow(non_snake_case)]
use crate::ffi::{IUnknown, BOOL, GUID, HRESULT, REFIID};
use std::ffi::c_void;

#[repr(C)]
pub struct IClassFactory<T> {
    pub CreateInstance: unsafe extern "system" fn(
        this: &mut T,
        pUnkOuter: *mut IUnknown<()>,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub LockServer: unsafe extern "system" fn(this: &mut T, fLock: BOOL) -> HRESULT,
}

impl IClassFactory<()> {
    // 00000001-0000-0000-C000-000000000046
    pub const IID: GUID = GUID {
        data1: 0x00000001,
        data2: 0x0000,
        data3: 0x0000,
        data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    };
}
