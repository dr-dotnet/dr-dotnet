#![allow(non_snake_case)]
use super::MetaDataImport;
use crate::ffi::{
    int, mdFieldDef, mdMethodDef, mdToken, mdTypeDef, AppDomainID, AssemblyID, ClassID, ContextID,
    CorElementType, CorProfilerFunctionEnum, CorProfilerMethodEnum, CorProfilerModuleEnum,
    CorProfilerObjectEnum, CorProfilerThreadEnum, FunctionEnter, FunctionEnter2, FunctionEnter3,
    FunctionEnter3WithInfo, FunctionID, FunctionIDMapper, FunctionIDMapper2, FunctionLeave,
    FunctionLeave2, FunctionLeave3, FunctionLeave3WithInfo, FunctionTailcall, FunctionTailcall2,
    FunctionTailcall3, FunctionTailcall3WithInfo, ICorProfilerInfo, ICorProfilerInfo10,
    ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6,
    ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, IUnknown, MethodMalloc, ModuleID,
    ObjectID, ObjectReferenceCallback, ProcessID, ReJITID, StackSnapshotCallback, ThreadID,
    Unknown, BOOL, BYTE, COR_DEBUG_IL_TO_NATIVE_MAP, COR_FIELD_OFFSET, COR_IL_MAP,
    COR_PRF_CODE_INFO, COR_PRF_ELT_INFO, COR_PRF_EX_CLAUSE_INFO, COR_PRF_FRAME_INFO,
    COR_PRF_FUNCTION_ARGUMENT_INFO, COR_PRF_FUNCTION_ARGUMENT_RANGE, COR_PRF_GC_GENERATION_RANGE,
    COR_PRF_RUNTIME_TYPE, COR_PRF_STATIC_TYPE, DWORD, HANDLE, HRESULT, LPCBYTE, PCCOR_SIGNATURE,
    REFIID, SIZE_T, UINT_PTR, ULONG, ULONG32, USHORT, WCHAR,
};
use std::ffi::c_void;
#[repr(C)]
pub struct CorProfilerInfoVtbl {
    pub IUnknown: IUnknown<CorProfilerInfo>,
    pub ICorProfilerInfo: ICorProfilerInfo<CorProfilerInfo>,
    pub ICorProfilerInfo2: ICorProfilerInfo2<CorProfilerInfo>,
    pub ICorProfilerInfo3: ICorProfilerInfo3<CorProfilerInfo>,
    pub ICorProfilerInfo4: ICorProfilerInfo4<CorProfilerInfo>,
    pub ICorProfilerInfo5: ICorProfilerInfo5<CorProfilerInfo>,
    pub ICorProfilerInfo6: ICorProfilerInfo6<CorProfilerInfo>,
    pub ICorProfilerInfo7: ICorProfilerInfo7<CorProfilerInfo>,
    pub ICorProfilerInfo8: ICorProfilerInfo8<CorProfilerInfo>,
    pub ICorProfilerInfo9: ICorProfilerInfo9<CorProfilerInfo>,
    pub ICorProfilerInfo10: ICorProfilerInfo10<CorProfilerInfo>,
}

#[derive(Clone)]
#[repr(C)]
pub struct CorProfilerInfo {
    pub lpVtbl: *const CorProfilerInfoVtbl,
}

impl CorProfilerInfo {
    unsafe fn i_cor_profiler_info(&self) -> &ICorProfilerInfo<Self> {
        &(*self.lpVtbl).ICorProfilerInfo
    }
    unsafe fn i_cor_profiler_info_2(&self) -> &ICorProfilerInfo2<Self> {
        &(*self.lpVtbl).ICorProfilerInfo2
    }
    unsafe fn i_cor_profiler_info_3(&self) -> &ICorProfilerInfo3<Self> {
        &(*self.lpVtbl).ICorProfilerInfo3
    }
    unsafe fn i_cor_profiler_info_4(&self) -> &ICorProfilerInfo4<Self> {
        &(*self.lpVtbl).ICorProfilerInfo4
    }
    unsafe fn i_cor_profiler_info_5(&self) -> &ICorProfilerInfo5<Self> {
        &(*self.lpVtbl).ICorProfilerInfo5
    }
    unsafe fn i_cor_profiler_info_6(&self) -> &ICorProfilerInfo6<Self> {
        &(*self.lpVtbl).ICorProfilerInfo6
    }
    unsafe fn i_cor_profiler_info_7(&self) -> &ICorProfilerInfo7<Self> {
        &(*self.lpVtbl).ICorProfilerInfo7
    }
    unsafe fn i_cor_profiler_info_8(&self) -> &ICorProfilerInfo8<Self> {
        &(*self.lpVtbl).ICorProfilerInfo8
    }
    unsafe fn i_cor_profiler_info_9(&self) -> &ICorProfilerInfo9<Self> {
        &(*self.lpVtbl).ICorProfilerInfo9
    }
    unsafe fn i_cor_profiler_info_10(&self) -> &ICorProfilerInfo10<Self> {
        &(*self.lpVtbl).ICorProfilerInfo10
    }
    pub unsafe fn GetClassFromObject(&self, objectId: ObjectID, pClassId: *mut ClassID) -> HRESULT {
        (self.i_cor_profiler_info().GetClassFromObject)(self, objectId, pClassId)
    }
    pub unsafe fn GetClassFromToken(
        &self,
        moduleId: ModuleID,
        typeDef: mdTypeDef,
        pClassId: *mut ClassID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetClassFromToken)(self, moduleId, typeDef, pClassId)
    }
    pub unsafe fn GetCodeInfo(
        &self,
        functionId: FunctionID,
        pStart: *mut LPCBYTE,
        pcSize: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetCodeInfo)(self, functionId, pStart, pcSize)
    }
    pub unsafe fn GetEventMask(&self, pdwEvents: *mut DWORD) -> HRESULT {
        (self.i_cor_profiler_info().GetEventMask)(self, pdwEvents)
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: LPCBYTE, pFunctionId: *mut FunctionID) -> HRESULT {
        (self.i_cor_profiler_info().GetFunctionFromIP)(self, ip, pFunctionId)
    }
    pub unsafe fn GetFunctionFromToken(
        &self,
        moduleId: ModuleID,
        token: mdToken,
        pFunctionId: *mut FunctionID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetFunctionFromToken)(self, moduleId, token, pFunctionId)
    }
    pub unsafe fn GetHandleFromThread(&self, threadId: ThreadID, phThread: *mut HANDLE) -> HRESULT {
        (self.i_cor_profiler_info().GetHandleFromThread)(self, threadId, phThread)
    }
    pub unsafe fn GetObjectSize(&self, objectId: ObjectID, pcSize: *mut ULONG) -> HRESULT {
        (self.i_cor_profiler_info().GetObjectSize)(self, objectId, pcSize)
    }
    pub unsafe fn IsArrayClass(
        &self,
        classId: ClassID,
        pBaseElemType: *mut CorElementType,
        pBaseClassId: *mut ClassID,
        pcRank: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info().IsArrayClass)(
            self,
            classId,
            pBaseElemType,
            pBaseClassId,
            pcRank,
        )
    }
    pub unsafe fn GetThreadInfo(
        &self,
        threadId: ThreadID,
        pdwWin32ThreadId: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetThreadInfo)(self, threadId, pdwWin32ThreadId)
    }
    pub unsafe fn GetCurrentThreadID(&self, pThreadId: *mut ThreadID) -> HRESULT {
        (self.i_cor_profiler_info().GetCurrentThreadID)(self, pThreadId)
    }
    pub unsafe fn GetClassIDInfo(
        &self,
        classId: ClassID,
        pModuleId: *mut ModuleID,
        pTypeDefToken: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetClassIDInfo)(self, classId, pModuleId, pTypeDefToken)
    }
    pub unsafe fn GetFunctionInfo(
        &self,
        functionId: FunctionID,
        pClassId: *mut ClassID,
        pModuleId: *mut ModuleID,
        pToken: *mut mdToken,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetFunctionInfo)(self, functionId, pClassId, pModuleId, pToken)
    }
    pub unsafe fn SetEventMask(&self, dwEvents: DWORD) -> HRESULT {
        (self.i_cor_profiler_info().SetEventMask)(self, dwEvents)
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(
        &self,
        pFuncEnter: *const FunctionEnter,
        pFuncLeave: *const FunctionLeave,
        pFuncTailcall: *const FunctionTailcall,
    ) -> HRESULT {
        (self.i_cor_profiler_info().SetEnterLeaveFunctionHooks)(
            self,
            pFuncEnter,
            pFuncLeave,
            pFuncTailcall,
        )
    }
    pub unsafe fn SetFunctionIDMapper(&self, pFunc: *const FunctionIDMapper) -> HRESULT {
        (self.i_cor_profiler_info().SetFunctionIDMapper)(self, pFunc)
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(
        &self,
        functionId: FunctionID,
        riid: REFIID,
        ppImport: *mut *mut MetaDataImport,
        pToken: *mut mdToken,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetTokenAndMetaDataFromFunction)(
            self, functionId, riid, ppImport, pToken,
        )
    }
    pub unsafe fn GetModuleInfo(
        &self,
        moduleId: ModuleID,
        ppBaseLoadAddress: *mut LPCBYTE,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAssemblyId: *mut AssemblyID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetModuleInfo)(
            self,
            moduleId,
            ppBaseLoadAddress,
            cchName,
            pcchName,
            szName,
            pAssemblyId,
        )
    }
    pub unsafe fn GetModuleMetaData(
        &self,
        moduleId: ModuleID,
        dwOpenFlags: DWORD,
        riid: REFIID,
        // I think this needs to be a coclass that implements one of these metadata interfaces: https://docs.microsoft.com/en-us/windows/win32/api/rometadataapi/
        ppOut: *mut *mut MetaDataImport,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetModuleMetaData)(self, moduleId, dwOpenFlags, riid, ppOut)
    }
    pub unsafe fn GetILFunctionBody(
        &self,
        moduleId: ModuleID,
        methodId: mdMethodDef,
        ppMethodHeader: *mut LPCBYTE,
        pcbMethodSize: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetILFunctionBody)(
            self,
            moduleId,
            methodId,
            ppMethodHeader,
            pcbMethodSize,
        )
    }
    pub unsafe fn GetILFunctionBodyAllocator(
        &self,
        moduleId: ModuleID,
        ppMalloc: *mut *mut MethodMalloc,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetILFunctionBodyAllocator)(self, moduleId, ppMalloc)
    }
    pub unsafe fn SetILFunctionBody(
        &self,
        moduleId: ModuleID,
        methodid: mdMethodDef, // methodid typo exists in corprof.idl
        pbNewILMethodHeader: LPCBYTE,
    ) -> HRESULT {
        (self.i_cor_profiler_info().SetILFunctionBody)(
            self,
            moduleId,
            methodid,
            pbNewILMethodHeader,
        )
    }
    pub unsafe fn GetAppDomainInfo(
        &self,
        appDomainId: AppDomainID,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pProcessId: *mut ProcessID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetAppDomainInfo)(
            self,
            appDomainId,
            cchName,
            pcchName,
            szName,
            pProcessId,
        )
    }
    pub unsafe fn GetAssemblyInfo(
        &self,
        assemblyId: AssemblyID,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAppDomainId: *mut AppDomainID,
        pModuleId: *mut ModuleID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetAssemblyInfo)(
            self,
            assemblyId,
            cchName,
            pcchName,
            szName,
            pAppDomainId,
            pModuleId,
        )
    }
    pub unsafe fn SetFunctionReJIT(&self, functionId: FunctionID) -> HRESULT {
        (self.i_cor_profiler_info().SetFunctionReJIT)(self, functionId)
    }
    pub unsafe fn ForceGC(&self) -> HRESULT {
        (self.i_cor_profiler_info().ForceGC)(self)
    }
    pub unsafe fn SetILInstrumentedCodeMap(
        &self,
        functionId: FunctionID,
        fStartJit: BOOL,
        cILMapEntries: ULONG,
        rgILMapEntries: *const COR_IL_MAP,
    ) -> HRESULT {
        (self.i_cor_profiler_info().SetILInstrumentedCodeMap)(
            self,
            functionId,
            fStartJit,
            cILMapEntries,
            rgILMapEntries,
        )
    }
    pub unsafe fn GetInprocInspectionInterface(
        &self,
        ppicd: *mut *mut Unknown, // TODO: Implement ICorDebugProcess and CorDebugProcess co-class
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetInprocInspectionInterface)(self, ppicd)
    }
    pub unsafe fn GetInprocInspectionIThisThread(
        &self,
        ppicd: *mut *mut Unknown, // TODO: Implement ICorDebugProcess and CorDebugProcess co-class
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetInprocInspectionIThisThread)(self, ppicd)
    }
    pub unsafe fn GetThreadContext(
        &self,
        threadId: ThreadID,
        pContextId: *mut ContextID,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetThreadContext)(self, threadId, pContextId)
    }
    pub unsafe fn BeginInprocDebugging(
        &self,
        fThisThreadOnly: BOOL,
        pdwProfilerContext: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info().BeginInprocDebugging)(self, fThisThreadOnly, pdwProfilerContext)
    }
    pub unsafe fn EndInprocDebugging(&self, dwProfilerContext: DWORD) -> HRESULT {
        (self.i_cor_profiler_info().EndInprocDebugging)(self, dwProfilerContext)
    }
    pub unsafe fn GetILToNativeMapping(
        &self,
        functionId: FunctionID,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT {
        (self.i_cor_profiler_info().GetILToNativeMapping)(self, functionId, cMap, pcMap, map)
    }
    pub unsafe fn DoStackSnapshot(
        &self,
        thread: ThreadID,
        callback: *const StackSnapshotCallback,
        infoFlags: ULONG32,
        clientData: *const c_void,
        context: *const BYTE,
        contextSize: ULONG32,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().DoStackSnapshot)(
            self,
            thread,
            callback,
            infoFlags,
            clientData,
            context,
            contextSize,
        )
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(
        &self,
        pFuncEnter: *const FunctionEnter2,
        pFuncLeave: *const FunctionLeave2,
        pFuncTailcall: *const FunctionTailcall2,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().SetEnterLeaveFunctionHooks2)(
            self,
            pFuncEnter,
            pFuncLeave,
            pFuncTailcall,
        )
    }
    pub unsafe fn GetFunctionInfo2(
        &self,
        funcId: FunctionID,
        frameInfo: COR_PRF_FRAME_INFO,
        pClassId: *mut ClassID,
        pModuleId: *mut ModuleID,
        pToken: *mut mdToken,
        cTypeArgs: ULONG32,
        pcTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetFunctionInfo2)(
            self, funcId, frameInfo, pClassId, pModuleId, pToken, cTypeArgs, pcTypeArgs, typeArgs,
        )
    }
    pub unsafe fn GetStringLayout(
        &self,
        pBufferLengthOffset: *mut ULONG,
        pStringLengthOffset: *mut ULONG,
        pBufferOffset: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetStringLayout)(
            self,
            pBufferLengthOffset,
            pStringLengthOffset,
            pBufferOffset,
        )
    }
    pub unsafe fn GetClassLayout(
        &self,
        classID: ClassID,
        rFieldOffset: *mut COR_FIELD_OFFSET,
        cFieldOffset: ULONG,
        pcFieldOffset: *mut ULONG,
        pulClassSize: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetClassLayout)(
            self,
            classID,
            rFieldOffset,
            cFieldOffset,
            pcFieldOffset,
            pulClassSize,
        )
    }
    pub unsafe fn GetClassIDInfo2(
        &self,
        classId: ClassID,
        pModuleId: *mut ModuleID,
        pTypeDefToken: *mut mdTypeDef,
        pParentClassId: *mut ClassID,
        cNumTypeArgs: ULONG32,
        pcNumTypeArgs: *mut ULONG32,
        typeArgs: *mut ClassID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetClassIDInfo2)(
            self,
            classId,
            pModuleId,
            pTypeDefToken,
            pParentClassId,
            cNumTypeArgs,
            pcNumTypeArgs,
            typeArgs,
        )
    }
    pub unsafe fn GetCodeInfo2(
        &self,
        functionID: FunctionID,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetCodeInfo2)(
            self,
            functionID,
            cCodeInfos,
            pcCodeInfos,
            codeInfos,
        )
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(
        &self,
        moduleID: ModuleID,
        typeDef: mdTypeDef,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pClassID: *mut ClassID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetClassFromTokenAndTypeArgs)(
            self, moduleID, typeDef, cTypeArgs, typeArgs, pClassID,
        )
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(
        &self,
        moduleID: ModuleID,
        funcDef: mdMethodDef,
        classId: ClassID,
        cTypeArgs: ULONG32,
        typeArgs: *const ClassID,
        pFunctionID: *mut FunctionID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetFunctionFromTokenAndTypeArgs)(
            self,
            moduleID,
            funcDef,
            classId,
            cTypeArgs,
            typeArgs,
            pFunctionID,
        )
    }
    pub unsafe fn EnumModuleFrozenObjects(
        &self,
        moduleID: ModuleID,
        ppEnum: *mut *mut CorProfilerObjectEnum,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().EnumModuleFrozenObjects)(self, moduleID, ppEnum)
    }
    pub unsafe fn GetArrayObjectInfo(
        &self,
        objectId: ObjectID,
        cDimensions: ULONG32,
        pDimensionSizes: *mut ULONG32,
        pDimensionLowerBounds: *mut int,
        ppData: *mut *mut BYTE,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetArrayObjectInfo)(
            self,
            objectId,
            cDimensions,
            pDimensionSizes,
            pDimensionLowerBounds,
            ppData,
        )
    }
    pub unsafe fn GetBoxClassLayout(
        &self,
        classId: ClassID,
        pBufferOffset: *mut ULONG32,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetBoxClassLayout)(self, classId, pBufferOffset)
    }
    pub unsafe fn GetThreadAppDomain(
        &self,
        threadId: ThreadID,
        pAppDomainId: *mut AppDomainID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetThreadAppDomain)(self, threadId, pAppDomainId)
    }
    pub unsafe fn GetRVAStaticAddress(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetRVAStaticAddress)(self, classId, fieldToken, ppAddress)
    }
    pub unsafe fn GetAppDomainStaticAddress(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        appDomainId: AppDomainID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetAppDomainStaticAddress)(
            self,
            classId,
            fieldToken,
            appDomainId,
            ppAddress,
        )
    }
    pub unsafe fn GetThreadStaticAddress(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        threadId: ThreadID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetThreadStaticAddress)(
            self, classId, fieldToken, threadId, ppAddress,
        )
    }
    pub unsafe fn GetContextStaticAddress(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        contextId: ContextID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetContextStaticAddress)(
            self, classId, fieldToken, contextId, ppAddress,
        )
    }
    pub unsafe fn GetStaticFieldInfo(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        pFieldInfo: *mut COR_PRF_STATIC_TYPE,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetStaticFieldInfo)(self, classId, fieldToken, pFieldInfo)
    }
    pub unsafe fn GetGenerationBounds(
        &self,
        cObjectRanges: ULONG,
        pcObjectRanges: *mut ULONG,
        ranges: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetGenerationBounds)(
            self,
            cObjectRanges,
            pcObjectRanges,
            ranges,
        )
    }
    pub unsafe fn GetObjectGeneration(
        &self,
        objectId: ObjectID,
        range: *mut COR_PRF_GC_GENERATION_RANGE,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetObjectGeneration)(self, objectId, range)
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(
        &self,
        pinfo: *mut COR_PRF_EX_CLAUSE_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_2().GetNotifiedExceptionClauseInfo)(self, pinfo)
    }
    pub unsafe fn EnumJITedFunctions(&self, ppEnum: *mut *mut CorProfilerFunctionEnum) -> HRESULT {
        (self.i_cor_profiler_info_3().EnumJITedFunctions)(self, ppEnum)
    }
    pub unsafe fn RequestProfilerDetach(&self, dwExpectedCompletionMilliseconds: DWORD) -> HRESULT {
        (self.i_cor_profiler_info_3().RequestProfilerDetach)(self, dwExpectedCompletionMilliseconds)
    }
    pub unsafe fn SetFunctionIDMapper2(
        &self,
        pFunc: *const FunctionIDMapper2,
        clientData: *const c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().SetFunctionIDMapper2)(self, pFunc, clientData)
    }
    pub unsafe fn GetStringLayout2(
        &self,
        pStringLengthOffset: *mut ULONG,
        pBufferOffset: *mut ULONG,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetStringLayout2)(self, pStringLengthOffset, pBufferOffset)
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(
        &self,
        pFuncEnter3: *const FunctionEnter3,
        pFuncLeave3: *const FunctionLeave3,
        pFuncTailcall3: *const FunctionTailcall3,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().SetEnterLeaveFunctionHooks3)(
            self,
            pFuncEnter3,
            pFuncLeave3,
            pFuncTailcall3,
        )
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(
        &self,
        pFuncEnter3WithInfo: *const FunctionEnter3WithInfo,
        pFuncLeave3WithInfo: *const FunctionLeave3WithInfo,
        pFuncTailcall3WithInfo: *const FunctionTailcall3WithInfo,
    ) -> HRESULT {
        (self
            .i_cor_profiler_info_3()
            .SetEnterLeaveFunctionHooks3WithInfo)(
            self,
            pFuncEnter3WithInfo,
            pFuncLeave3WithInfo,
            pFuncTailcall3WithInfo,
        )
    }
    pub unsafe fn GetFunctionEnter3Info(
        &self,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
        pcbArgumentInfo: *mut ULONG,
        pArgumentInfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetFunctionEnter3Info)(
            self,
            functionId,
            eltInfo,
            pFrameInfo,
            pcbArgumentInfo,
            pArgumentInfo,
        )
    }
    pub unsafe fn GetFunctionLeave3Info(
        &self,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
        pRetvalRange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetFunctionLeave3Info)(
            self,
            functionId,
            eltInfo,
            pFrameInfo,
            pRetvalRange,
        )
    }
    pub unsafe fn GetFunctionTailcall3Info(
        &self,
        functionId: FunctionID,
        eltInfo: COR_PRF_ELT_INFO,
        pFrameInfo: *mut COR_PRF_FRAME_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetFunctionTailcall3Info)(
            self, functionId, eltInfo, pFrameInfo,
        )
    }
    pub unsafe fn EnumModules(&self, ppEnum: *mut *mut CorProfilerModuleEnum) -> HRESULT {
        (self.i_cor_profiler_info_3().EnumModules)(self, ppEnum)
    }
    pub unsafe fn GetRuntimeInformation(
        &self,
        pClrInstanceId: *mut USHORT,
        pRuntimeType: *mut COR_PRF_RUNTIME_TYPE,
        pMajorVersion: *mut USHORT,
        pMinorVersion: *mut USHORT,
        pBuildNumber: *mut USHORT,
        pQFEVersion: *mut USHORT,
        cchVersionString: ULONG,
        pcchVersionString: *mut ULONG,
        szVersionString: *mut WCHAR,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetRuntimeInformation)(
            self,
            pClrInstanceId,
            pRuntimeType,
            pMajorVersion,
            pMinorVersion,
            pBuildNumber,
            pQFEVersion,
            cchVersionString,
            pcchVersionString,
            szVersionString,
        )
    }
    pub unsafe fn GetThreadStaticAddress2(
        &self,
        classId: ClassID,
        fieldToken: mdFieldDef,
        appDomainId: AppDomainID,
        threadId: ThreadID,
        ppAddress: *mut *mut c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetThreadStaticAddress2)(
            self,
            classId,
            fieldToken,
            appDomainId,
            threadId,
            ppAddress,
        )
    }
    pub unsafe fn GetAppDomainsContainingModule(
        &self,
        moduleId: ModuleID,
        cAppDomainIds: ULONG32,
        pcAppDomainIds: *mut ULONG32,
        appDomainIds: *mut AppDomainID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetAppDomainsContainingModule)(
            self,
            moduleId,
            cAppDomainIds,
            pcAppDomainIds,
            appDomainIds,
        )
    }
    pub unsafe fn GetModuleInfo2(
        &self,
        moduleId: ModuleID,
        ppBaseLoadAddress: *mut LPCBYTE,
        cchName: ULONG,
        pcchName: *mut ULONG,
        szName: *mut WCHAR,
        pAssemblyId: *mut AssemblyID,
        pdwModuleFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info_3().GetModuleInfo2)(
            self,
            moduleId,
            ppBaseLoadAddress,
            cchName,
            pcchName,
            szName,
            pAssemblyId,
            pdwModuleFlags,
        )
    }
    pub unsafe fn EnumThreads(&self, ppEnum: *mut *mut CorProfilerThreadEnum) -> HRESULT {
        (self.i_cor_profiler_info_4().EnumThreads)(self, ppEnum)
    }
    pub unsafe fn InitializeCurrentThread(&self) -> HRESULT {
        (self.i_cor_profiler_info_4().InitializeCurrentThread)(self)
    }
    pub unsafe fn RequestReJIT(
        &self,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().RequestReJIT)(self, cFunctions, moduleIds, methodIds)
    }
    pub unsafe fn RequestRevert(
        &self,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
        status: *mut HRESULT,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().RequestRevert)(self, cFunctions, moduleIds, methodIds, status)
    }
    pub unsafe fn GetCodeInfo3(
        &self,
        functionID: FunctionID,
        reJitId: ReJITID,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().GetCodeInfo3)(
            self,
            functionID,
            reJitId,
            cCodeInfos,
            pcCodeInfos,
            codeInfos,
        )
    }
    pub unsafe fn GetFunctionFromIP2(
        &self,
        ip: LPCBYTE,
        pFunctionId: *mut FunctionID,
        pReJitId: *mut ReJITID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().GetFunctionFromIP2)(self, ip, pFunctionId, pReJitId)
    }
    pub unsafe fn GetReJITIDs(
        &self,
        functionId: FunctionID,
        cReJitIds: ULONG,
        pcReJitIds: *mut ULONG,
        reJitIds: *mut ReJITID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().GetReJITIDs)(
            self, functionId, cReJitIds, pcReJitIds, reJitIds,
        )
    }
    pub unsafe fn GetILToNativeMapping2(
        &self,
        functionId: FunctionID,
        reJitId: ReJITID,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT {
        (self.i_cor_profiler_info_4().GetILToNativeMapping2)(
            self, functionId, reJitId, cMap, pcMap, map,
        )
    }
    pub unsafe fn EnumJITedFunctions2(&self, ppEnum: *mut *mut CorProfilerFunctionEnum) -> HRESULT {
        (self.i_cor_profiler_info_4().EnumJITedFunctions2)(self, ppEnum)
    }
    pub unsafe fn GetObjectSize2(&self, objectId: ObjectID, pcSize: *mut SIZE_T) -> HRESULT {
        (self.i_cor_profiler_info_4().GetObjectSize2)(self, objectId, pcSize)
    }
    pub unsafe fn GetEventMask2(
        &self,
        pdwEventsLow: *mut DWORD,
        pdwEventsHigh: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info_5().GetEventMask2)(self, pdwEventsLow, pdwEventsHigh)
    }
    pub unsafe fn SetEventMask2(&self, dwEventsLow: DWORD, dwEventsHigh: DWORD) -> HRESULT {
        (self.i_cor_profiler_info_5().SetEventMask2)(self, dwEventsLow, dwEventsHigh)
    }
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(
        &self,
        inlinersModuleId: ModuleID,
        inlineeModuleId: ModuleID,
        inlineeMethodId: mdMethodDef,
        incompleteData: *mut BOOL,
        ppEnum: *mut *mut CorProfilerMethodEnum,
    ) -> HRESULT {
        (self
            .i_cor_profiler_info_6()
            .EnumNgenModuleMethodsInliningThisMethod)(
            self,
            inlinersModuleId,
            inlineeModuleId,
            inlineeMethodId,
            incompleteData,
            ppEnum,
        )
    }
    pub unsafe fn ApplyMetaData(&self, moduleId: ModuleID) -> HRESULT {
        (self.i_cor_profiler_info_7().ApplyMetaData)(self, moduleId)
    }
    pub unsafe fn GetInMemorySymbolsLength(
        &self,
        moduleId: ModuleID,
        pCountSymbolBytes: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info_7().GetInMemorySymbolsLength)(self, moduleId, pCountSymbolBytes)
    }
    pub unsafe fn ReadInMemorySymbols(
        &self,
        moduleId: ModuleID,
        symbolsReadOffset: DWORD,
        pSymbolBytes: *mut BYTE,
        countSymbolBytes: DWORD,
        pCountSymbolBytesRead: *mut DWORD,
    ) -> HRESULT {
        (self.i_cor_profiler_info_7().ReadInMemorySymbols)(
            self,
            moduleId,
            symbolsReadOffset,
            pSymbolBytes,
            countSymbolBytes,
            pCountSymbolBytesRead,
        )
    }
    pub unsafe fn IsFunctionDynamic(
        &self,
        functionId: FunctionID,
        isDynamic: *mut BOOL,
    ) -> HRESULT {
        (self.i_cor_profiler_info_8().IsFunctionDynamic)(self, functionId, isDynamic)
    }
    pub unsafe fn GetFunctionFromIP3(
        &self,
        ip: LPCBYTE,
        functionId: *mut FunctionID,
        pReJitId: *mut ReJITID,
    ) -> HRESULT {
        (self.i_cor_profiler_info_8().GetFunctionFromIP3)(self, ip, functionId, pReJitId)
    }
    pub unsafe fn GetDynamicFunctionInfo(
        &self,
        functionId: FunctionID,
        moduleId: *mut ModuleID,
        ppvSig: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
        cchName: ULONG,
        pcchName: *mut ULONG,
        wszName: *mut WCHAR,
    ) -> HRESULT {
        (self.i_cor_profiler_info_8().GetDynamicFunctionInfo)(
            self, functionId, moduleId, ppvSig, pbSig, cchName, pcchName, wszName,
        )
    }
    pub unsafe fn GetNativeCodeStartAddresses(
        &self,
        functionID: FunctionID,
        reJitId: ReJITID,
        cCodeStartAddresses: ULONG32,
        pcCodeStartAddresses: *mut ULONG32,
        codeStartAddresses: *mut UINT_PTR,
    ) -> HRESULT {
        (self.i_cor_profiler_info_9().GetNativeCodeStartAddresses)(
            self,
            functionID,
            reJitId,
            cCodeStartAddresses,
            pcCodeStartAddresses,
            codeStartAddresses,
        )
    }
    pub unsafe fn GetILToNativeMapping3(
        &self,
        pNativeCodeStartAddress: UINT_PTR,
        cMap: ULONG32,
        pcMap: *mut ULONG32,
        map: *mut COR_DEBUG_IL_TO_NATIVE_MAP,
    ) -> HRESULT {
        (self.i_cor_profiler_info_9().GetILToNativeMapping3)(
            self,
            pNativeCodeStartAddress,
            cMap,
            pcMap,
            map,
        )
    }
    pub unsafe fn GetCodeInfo4(
        &self,
        pNativeCodeStartAddress: UINT_PTR,
        cCodeInfos: ULONG32,
        pcCodeInfos: *mut ULONG32,
        codeInfos: *mut COR_PRF_CODE_INFO,
    ) -> HRESULT {
        (self.i_cor_profiler_info_9().GetCodeInfo4)(
            self,
            pNativeCodeStartAddress,
            cCodeInfos,
            pcCodeInfos,
            codeInfos,
        )
    }
    pub unsafe fn EnumerateObjectReferences(
        &self,
        objectId: ObjectID,
        callback: ObjectReferenceCallback,
        clientData: *const c_void,
    ) -> HRESULT {
        (self.i_cor_profiler_info_10().EnumerateObjectReferences)(
            self, objectId, callback, clientData,
        )
    }
    pub unsafe fn IsFrozenObject(&self, objectId: ObjectID, pbFrozen: *mut BOOL) -> HRESULT {
        (self.i_cor_profiler_info_10().IsFrozenObject)(self, objectId, pbFrozen)
    }
    pub unsafe fn GetLOHObjectSizeThreshold(&self, pThreshold: *mut DWORD) -> HRESULT {
        (self.i_cor_profiler_info_10().GetLOHObjectSizeThreshold)(self, pThreshold)
    }
    pub unsafe fn RequestReJITWithInliners(
        &self,
        dwRejitFlags: DWORD,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
    ) -> HRESULT {
        (self.i_cor_profiler_info_10().RequestReJITWithInliners)(
            self,
            dwRejitFlags,
            cFunctions,
            moduleIds,
            methodIds,
        )
    }
    pub unsafe fn SuspendRuntime(&self) -> HRESULT {
        (self.i_cor_profiler_info_10().SuspendRuntime)(self)
    }
    pub unsafe fn ResumeRuntime(&self) -> HRESULT {
        (self.i_cor_profiler_info_10().ResumeRuntime)(self)
    }
}
