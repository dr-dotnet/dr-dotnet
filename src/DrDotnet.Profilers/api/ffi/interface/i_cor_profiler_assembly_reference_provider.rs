#![allow(non_snake_case)]
use crate::ffi::{COR_PRF_ASSEMBLY_REFERENCE_INFO, GUID, HRESULT};

#[repr(C)]
pub struct ICorProfilerAssemblyReferenceProvider<T> {
    pub AddAssemblyReference: unsafe extern "system" fn(
        this: &T,
        pAssemblyRefInfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO,
    ) -> HRESULT,
}

impl ICorProfilerAssemblyReferenceProvider<()> {
    // 66A78C24-2EEF-4F65-B45F-DD1D8038BF3C
    pub const IID: GUID = GUID {
        data1: 0x66A78C24,
        data2: 0x2EEF,
        data3: 0x4F65,
        data4: [0xB4, 0x5F, 0xDD, 0x1D, 0x80, 0x38, 0xBF, 0x3C],
    };
}
