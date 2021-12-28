#![allow(non_snake_case)]
use crate::ffi::{CorProfilerAssemblyReferenceProvider, GUID, HRESULT, WCHAR};

#[repr(C)]
pub struct ICorProfilerCallback6<T> {
    pub GetAssemblyReferences: unsafe extern "system" fn(
        this: &mut T,
        wszAssemblyPath: *const WCHAR,
        pAsmRefProvider: *const CorProfilerAssemblyReferenceProvider,
    ) -> HRESULT,
}

impl ICorProfilerCallback6<()> {
    // FC13DF4B-4448-4F4F-950C-BA8D19D00C36
    pub const IID: GUID = GUID {
        data1: 0xFC13DF4B,
        data2: 0x4448,
        data3: 0x4F4F,
        data4: [0x95, 0x0C, 0xBA, 0x8D, 0x19, 0xD0, 0x0C, 0x36],
    };
}
