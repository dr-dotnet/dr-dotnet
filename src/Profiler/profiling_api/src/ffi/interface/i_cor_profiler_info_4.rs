#![allow(non_snake_case)]
use crate::ffi::{
    mdMethodDef, CorProfilerFunctionEnum, CorProfilerThreadEnum, FunctionID, ModuleID, ObjectID,
    ReJITID, COR_DEBUG_IL_TO_NATIVE_MAP, COR_PRF_CODE_INFO, GUID, HRESULT, LPCBYTE, SIZE_T, ULONG,
    ULONG32,
};

#[repr(C)]
pub struct ICorProfilerInfo4<T> {
    pub EnumThreads:
        unsafe extern "system" fn(this: &T, ppEnum: *mut *mut CorProfilerThreadEnum) -> HRESULT,

    pub InitializeCurrentThread: unsafe extern "system" fn(this: &T) -> HRESULT,

    pub RequestReJIT: unsafe extern "system" fn(
        this: &T,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
    ) -> HRESULT,

    pub RequestRevert: unsafe extern "system" fn(
        this: &T,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
        status: *mut HRESULT,
    ) -> HRESULT,

    pub GetCodeInfo3: unsafe extern "system" fn(
        this: &T,
        functionID: FunctionID,
        reJitId: ReJITID,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT,

    pub GetFunctionFromIP2: unsafe extern "system" fn(
        this: &T,
        ip: LPCBYTE,
        pFunctionId: *mut FunctionID,
        pReJitId: *mut ReJITID,
    ) -> HRESULT,

    pub GetReJITIDs: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        cReJitIds: ULONG,
        pcReJitIds: *mut ULONG,
        reJitIds: *mut ReJITID,
    ) -> HRESULT,

    pub GetILToNativeMapping2: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        reJitId: ReJITID,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT,

    pub EnumJITedFunctions2:
        unsafe extern "system" fn(this: &T, ppEnum: *mut *mut CorProfilerFunctionEnum) -> HRESULT,

    pub GetObjectSize2:
        unsafe extern "system" fn(this: &T, objectId: ObjectID, pcSize: *mut SIZE_T) -> HRESULT,
}

impl ICorProfilerInfo4<()> {
    // 0D8FDCAA-6257-47BF-B1BF-94DAC88466EE
    pub const IID: GUID = GUID {
        data1: 0x0D8FDCAA,
        data2: 0x6257,
        data3: 0x47BF,
        data4: [0xB1, 0xBF, 0x94, 0xDA, 0xC8, 0x84, 0x66, 0xEE],
    };
}
