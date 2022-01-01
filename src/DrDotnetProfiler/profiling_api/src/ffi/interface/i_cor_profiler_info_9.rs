#![allow(non_snake_case)]
use crate::ffi::{
    FunctionID, ReJITID, COR_DEBUG_IL_TO_NATIVE_MAP, COR_PRF_CODE_INFO, GUID, HRESULT, UINT_PTR,
    ULONG32,
};

#[repr(C)]
pub struct ICorProfilerInfo9<T> {
    pub GetNativeCodeStartAddresses: unsafe extern "system" fn(
        this: &T,
        functionID: FunctionID,
        reJitId: ReJITID,
        cCodeStartAddresses: ULONG32,
        pcCodeStartAddresses: *mut ULONG32,
        codeStartAddresses: *mut UINT_PTR,
    ) -> HRESULT,
    pub GetILToNativeMapping3: unsafe extern "system" fn(
        this: &T,
        pNativeCodeStartAddress: UINT_PTR,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT,
    pub GetCodeInfo4: unsafe extern "system" fn(
        this: &T,
        pNativeCodeStartAddress: UINT_PTR,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT,
}

impl ICorProfilerInfo9<()> {
    // 008170DB-F8CC-4796-9A51-DC8AA0B47012
    pub const IID: GUID = GUID {
        data1: 0x008170DB,
        data2: 0xF8CC,
        data3: 0x4796,
        data4: [0x9A, 0x51, 0xDC, 0x8A, 0xA0, 0xB4, 0x70, 0x12],
    };
}
