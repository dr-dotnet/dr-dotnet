#![allow(non_snake_case)]
use crate::ffi::{
    mdMethodDef, mdToken, mdTypeDef, AppDomainID, AssemblyID, ClassID, ContextID, CorElementType,
    FunctionEnter, FunctionID, FunctionIDMapper, FunctionLeave, FunctionTailcall, MetaDataImport,
    MethodMalloc, ModuleID, ObjectID, ProcessID, ThreadID, Unknown, BOOL,
    COR_DEBUG_IL_TO_NATIVE_MAP, COR_IL_MAP, DWORD, GUID, HANDLE, HRESULT, LPCBYTE, REFIID, ULONG,
    ULONG32, WCHAR,
};

#[repr(C)]
pub struct ICorProfilerInfo<T> {
    pub GetClassFromObject:
        unsafe extern "system" fn(this: &T, objectId: ObjectID, pClassId: *mut ClassID) -> HRESULT,
    pub GetClassFromToken: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        typeDef: mdTypeDef,
        pClassId: *mut ClassID,
    ) -> HRESULT,
    pub GetCodeInfo: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        pStart: *mut LPCBYTE,
        pcSize: *mut ULONG,
    ) -> HRESULT,
    pub GetEventMask: unsafe extern "system" fn(this: &T, pdwEvents: *mut DWORD) -> HRESULT,
    pub GetFunctionFromIP:
        unsafe extern "system" fn(this: &T, ip: LPCBYTE, pFunctionId: *mut FunctionID) -> HRESULT,
    pub GetFunctionFromToken: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        token: mdToken,
        pFunctionId: *mut FunctionID,
    ) -> HRESULT,
    pub GetHandleFromThread:
        unsafe extern "system" fn(this: &T, threadId: ThreadID, phThread: *mut HANDLE) -> HRESULT,
    pub GetObjectSize:
        unsafe extern "system" fn(this: &T, objectId: ObjectID, pcSize: *mut ULONG) -> HRESULT,
    pub IsArrayClass: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pBaseElemType: *mut CorElementType,
        pBaseClassId: *mut ClassID,
        pcRank: *mut ULONG,
    ) -> HRESULT,
    pub GetThreadInfo: unsafe extern "system" fn(
        this: &T,
        threadId: ThreadID,
        pdwWin32ThreadId: *mut DWORD,
    ) -> HRESULT,
    pub GetCurrentThreadID:
        unsafe extern "system" fn(this: &T, pThreadId: *mut ThreadID) -> HRESULT,
    pub GetClassIDInfo: unsafe extern "system" fn(
        this: &T,
        classId: ClassID,
        pModuleId: *mut ModuleID,
        pTypeDefToken: *mut mdTypeDef,
    ) -> HRESULT,
    pub GetFunctionInfo: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        pClassId: *mut ClassID,
        pModuleId: *mut ModuleID,
        pToken: *mut mdToken,
    ) -> HRESULT,
    pub SetEventMask: unsafe extern "system" fn(this: &T, dwEvents: DWORD) -> HRESULT,
    pub SetEnterLeaveFunctionHooks: unsafe extern "system" fn(
        this: &T,
        pFuncEnter: *const FunctionEnter,
        pFuncLeave: *const FunctionLeave,
        pFuncTailcall: *const FunctionTailcall,
    ) -> HRESULT,
    pub SetFunctionIDMapper:
        unsafe extern "system" fn(this: &T, pFunc: *const FunctionIDMapper) -> HRESULT,
    pub GetTokenAndMetaDataFromFunction: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        riid: REFIID,
        ppImport: *mut *mut MetaDataImport,
        pToken: *mut mdToken,
    ) -> HRESULT,
    pub GetModuleInfo: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        ppBaseLoadAddress: *mut LPCBYTE,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAssemblyId: *mut AssemblyID,
    ) -> HRESULT,
    pub GetModuleMetaData: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        dwOpenFlags: DWORD,
        riid: REFIID,
        ppOut: *mut *mut MetaDataImport,
    ) -> HRESULT,
    pub GetILFunctionBody: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        methodId: mdMethodDef,
        ppMethodHeader: *mut LPCBYTE,
        pcbMethodSize: *mut ULONG,
    ) -> HRESULT,
    pub GetILFunctionBodyAllocator: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        ppMalloc: *mut *mut MethodMalloc,
    ) -> HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(
        this: &T,
        moduleId: ModuleID,
        methodid: mdMethodDef, // methodid typo exists in corprof.idl
        pbNewILMethodHeader: LPCBYTE,
    ) -> HRESULT,
    pub GetAppDomainInfo: unsafe extern "system" fn(
        this: &T,
        appDomainId: AppDomainID,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pProcessId: *mut ProcessID,
    ) -> HRESULT,
    pub GetAssemblyInfo: unsafe extern "system" fn(
        this: &T,
        assemblyId: AssemblyID,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAppDomainId: *mut AppDomainID,
        pModuleId: *mut ModuleID,
    ) -> HRESULT,
    pub SetFunctionReJIT: unsafe extern "system" fn(this: &T, functionId: FunctionID) -> HRESULT,
    pub ForceGC: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        fStartJit: BOOL,
        cILMapEntries: ULONG,
        rgILMapEntries: *const COR_IL_MAP,
    ) -> HRESULT,
    pub GetInprocInspectionInterface: unsafe extern "system" fn(
        this: &T,
        ppicd: *mut *mut Unknown, // Can query for ICorDebugProcess
    ) -> HRESULT,
    pub GetInprocInspectionIThisThread: unsafe extern "system" fn(
        this: &T,
        ppicd: *mut *mut Unknown, // Can query for ICorDebugThread
    ) -> HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(
        this: &T,
        threadId: ThreadID,
        pContextId: *mut ContextID,
    ) -> HRESULT,
    pub BeginInprocDebugging: unsafe extern "system" fn(
        this: &T,
        fThisThreadOnly: BOOL,
        pdwProfilerContext: *mut DWORD,
    ) -> HRESULT,
    pub EndInprocDebugging:
        unsafe extern "system" fn(this: &T, dwProfilerContext: DWORD) -> HRESULT,
    pub GetILToNativeMapping: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT,
}

impl ICorProfilerInfo<()> {
    // 28B5557D-3F3F-48b4-90B2-5F9EEA2F6C48
    pub const IID: GUID = GUID {
        data1: 0x28B5557D,
        data2: 0x3F3F,
        data3: 0x48b4,
        data4: [0x90, 0xB2, 0x5F, 0x9E, 0xEA, 0x2F, 0x6C, 0x48],
    };
}
