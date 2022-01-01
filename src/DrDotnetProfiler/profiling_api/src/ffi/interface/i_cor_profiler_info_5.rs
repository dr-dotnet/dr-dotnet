#![allow(non_snake_case)]
use crate::ffi::{DWORD, GUID, HRESULT};

#[repr(C)]
pub struct ICorProfilerInfo5<T> {
    pub GetEventMask2: unsafe extern "system" fn(
        this: &T,
        pdwEventsLow: *mut DWORD,
        pdwEventsHigh: *mut DWORD,
    ) -> HRESULT,
    pub SetEventMask2:
        unsafe extern "system" fn(this: &T, dwEventsLow: DWORD, dwEventsHigh: DWORD) -> HRESULT,
}

impl ICorProfilerInfo5<()> {
    // 07602928-CE38-4B83-81E7-74ADAF781214
    pub const IID: GUID = GUID {
        data1: 0x07602928,
        data2: 0xCE38,
        data3: 0x4B83,
        data4: [0x81, 0xE7, 0x74, 0xAD, 0xAF, 0x78, 0x12, 0x14],
    };
}
