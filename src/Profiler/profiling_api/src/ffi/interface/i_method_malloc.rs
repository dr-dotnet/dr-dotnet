#![allow(non_snake_case)]
use crate::ffi::{GUID, HRESULT, ULONG};

#[repr(C)]
pub struct IMethodMalloc<T> {
    pub Alloc: unsafe extern "system" fn(this: &T, cb: ULONG) -> HRESULT,
}

impl IMethodMalloc<()> {
    // A0EFB28B-6EE2-4D7B-B983-A75EF7BEEDB8
    pub const IID: GUID = GUID {
        data1: 0xA0EFB28B,
        data2: 0x6EE2,
        data3: 0x4D7B,
        data4: [0xB9, 0x83, 0xA7, 0x5E, 0xF7, 0xBE, 0xED, 0xB8],
    };
}
