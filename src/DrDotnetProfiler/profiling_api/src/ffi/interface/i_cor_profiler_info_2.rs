#![allow(non_snake_case)]
use crate::ffi::{
    int, mdFieldDef, mdMethodDef, mdToken, mdTypeDef, AppDomainID, ClassID, ContextID,
    CorProfilerObjectEnum, FunctionEnter2, FunctionID, FunctionLeave2, FunctionTailcall2, ModuleID,
    ObjectID, StackSnapshotCallback, ThreadID, BYTE, COR_FIELD_OFFSET, COR_PRF_CODE_INFO,
    COR_PRF_EX_CLAUSE_INFO, COR_PRF_FRAME_INFO, COR_PRF_GC_GENERATION_RANGE, COR_PRF_STATIC_TYPE,
    GUID, HRESULT, ULONG, ULONG32,
};
use std::ffi::c_void;

#[repr(C)]
pub struct ICorProfilerInfo2<T> {
    pub DoStackSnapshot: unsafe extern "system" fn(
        this: &T,
        thread: ThreadID,
        callback: *const StackSnapshotCallback,
        infoFlags: ULONG32,
        clientData: *const c_void,
        context: *const BYTE,
        contextSize: ULONG32,
    ) -> HRESULT,
    pub SetEnterLeaveFunctionHooks2: unsafe extern "system" fn(
        this: &T,
        pFuncEnter: *const FunctionEnter2,
        pFuncLeave: *const FunctionLeave2,
        pFuncTailcall: *const FunctionTailcall2,
    ) -> HRESULT,
    pub GetFunctionInfo2: unsafe extern "system" fn(
        this: &T,
        funcId: FunctionID,
        frameInfo: COR_PRF_FRAME_INFO,
        pClassId: *mut ClassID,
        pModuleId: *mut ModuleID,
        pToken: *mut mdToken,
        cTypeArgs: ULONG32,
        pcTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT,
    pub GetStringLayout: unsafe extern "system" fn(
        this: &T,
        pBufferLengthOffset: *mut ULONG,
        pStringLengthOffset: *mut ULONG,
        pBufferOffset: *mut ULONG,
    ) -> HRESULT,
    pub GetClassLayout: unsafe extern "system" fn(
        this: &T,
        classID: ClassID,
        rFieldOffset: *mut COR_FIELD_OFFSET,
        cFieldOffset: ULONG,
        pcFieldOffset: *mut ULONG,
        pulClassSize: *mut ULONG,
    ) -> HRESULT,
    pub GetClassIDInfo2: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pModuleId: *mut ModuleID,
        pTypeDefToken: *mut mdTypeDef,
        pParentClassId: *mut ClassID,
        cNumTypeArgs: ULONG32,
        pcNumTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT,
    pub GetCodeInfo2: unsafe extern "system" fn(
        this: &T,
        functionID: FunctionID,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT,
    pub GetClassFromTokenAndTypeArgs: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        typeDef: mdTypeDef,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pClassID: *mut ClassID,
    ) -> HRESULT,
    pub GetFunctionFromTokenAndTypeArgs: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        funcDef: mdMethodDef,
        classId: ClassID,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pFunctionID: *mut FunctionID,
    ) -> HRESULT,
    pub EnumModuleFrozenObjects: unsafe extern "system" fn(
        this: &T,
        moduleID: ModuleID,
        ppEnum: *mut *mut CorProfilerObjectEnum,
    ) -> HRESULT,
    pub GetArrayObjectInfo: unsafe extern "system" fn(
        this: &T,
        objectId: ObjectID,
        cDimensions: ULONG32,
        pDimensionSizes: *mut ULONG32,
        pDimensionLowerBounds: *mut int,
        ppData: *mut *mut BYTE,
    ) -> HRESULT,
    pub GetBoxClassLayout: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pBufferOffset: *mut ULONG32,
    ) -> HRESULT,
    pub GetThreadAppDomain: unsafe extern "system" fn(
        this: &T,
        threadId: ThreadID,
        pAppDomainId: *mut AppDomainID,
    ) -> HRESULT,
    pub GetRVAStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetAppDomainStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        appDomainId: AppDomainID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetThreadStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        threadId: ThreadID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetContextStaticAddress: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        contextId: ContextID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT,
    pub GetStaticFieldInfo: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        fieldToken: mdFieldDef,
        pFieldInfo: *mut COR_PRF_STATIC_TYPE,
    ) -> HRESULT,
    pub GetGenerationBounds: unsafe extern "system" fn(
        this: &T,
        cObjectRanges: ULONG,
        pcObjectRanges: *mut ULONG,
        ranges: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT,
    pub GetObjectGeneration: unsafe extern "system" fn(
        this: &T,
        objectId: ObjectID,
        range: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT,
    pub GetNotifiedExceptionClauseInfo:
        unsafe extern "system" fn(this: &T, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> HRESULT,
}

impl ICorProfilerInfo2<()> {
    // CC0935CD-A518-487d-B0BB-A93214E65478
    pub const IID: GUID = GUID {
        data1: 0xCC0935CD,
        data2: 0xA518,
        data3: 0x487d,
        data4: [0xB0, 0xBB, 0xA9, 0x32, 0x14, 0xE6, 0x54, 0x78],
    };
}
