#![allow(non_snake_case)]
use crate::ffi::{CorProfilerInfo, GUID, HRESULT, UINT};
use std::ffi::c_void;

#[repr(C)]
pub struct ICorProfilerCallback3<T> {
    pub InitializeForAttach: unsafe extern "system" fn(
        this: &mut T,
        pCorProfilerInfoUnk: *const CorProfilerInfo,
        pvClientData: *const c_void,
        cbClientData: UINT,
    ) -> HRESULT,
    pub ProfilerAttachComplete: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ProfilerDetachSucceeded: unsafe extern "system" fn(this: &mut T) -> HRESULT,
}

impl ICorProfilerCallback3<()> {
    // 4FD2ED52-7731-4B8D-9469-03D2CC3086C5
    pub const IID: GUID = GUID {
        data1: 0x4FD2ED52,
        data2: 0x7731,
        data3: 0x4B8D,
        data4: [0x94, 0x69, 0x03, 0xD2, 0xCC, 0x30, 0x86, 0xC5],
    };
}
