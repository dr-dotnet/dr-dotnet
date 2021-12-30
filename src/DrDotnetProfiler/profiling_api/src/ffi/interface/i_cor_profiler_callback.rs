#![allow(non_snake_case)]
use crate::ffi::{
    AppDomainID, AssemblyID, ClassID, CorProfilerInfo, FunctionID, ModuleID, ObjectID, ThreadID,
    BOOL, COR_PRF_JIT_CACHE, COR_PRF_SUSPEND_REASON, COR_PRF_TRANSITION_REASON, DWORD, GUID,
    HRESULT, REFGUID, UINT_PTR, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct ICorProfilerCallback<T> {
    pub Initialize: unsafe extern "system" fn(
        this: &mut T,
        pICorProfilerInfoUnk: *const CorProfilerInfo,
    ) -> HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub AppDomainCreationStarted:
        unsafe extern "system" fn(this: &mut T, appDomainId: AppDomainID) -> HRESULT,
    pub AppDomainCreationFinished: unsafe extern "system" fn(
        this: &mut T,
        appDomainId: AppDomainID,
        hrStatus: HRESULT,
    ) -> HRESULT,
    pub AppDomainShutdownStarted:
        unsafe extern "system" fn(this: &mut T, appDomainId: AppDomainID) -> HRESULT,
    pub AppDomainShutdownFinished: unsafe extern "system" fn(
        this: &mut T,
        appDomainId: AppDomainID,
        hrStatus: HRESULT,
    ) -> HRESULT,
    pub AssemblyLoadStarted:
        unsafe extern "system" fn(this: &mut T, assemblyId: AssemblyID) -> HRESULT,
    pub AssemblyLoadFinished: unsafe extern "system" fn(
        this: &mut T,
        assemblyId: AssemblyID,
        hrStatus: HRESULT,
    ) -> HRESULT,
    pub AssemblyUnloadStarted:
        unsafe extern "system" fn(this: &mut T, assemblyId: AssemblyID) -> HRESULT,
    pub AssemblyUnloadFinished: unsafe extern "system" fn(
        this: &mut T,
        assemblyId: AssemblyID,
        hrStatus: HRESULT,
    ) -> HRESULT,
    pub ModuleLoadStarted: unsafe extern "system" fn(this: &mut T, moduleId: ModuleID) -> HRESULT,
    pub ModuleLoadFinished:
        unsafe extern "system" fn(this: &mut T, moduleId: ModuleID, hrStatus: HRESULT) -> HRESULT,
    pub ModuleUnloadStarted: unsafe extern "system" fn(this: &mut T, moduleId: ModuleID) -> HRESULT,
    pub ModuleUnloadFinished:
        unsafe extern "system" fn(this: &mut T, moduleId: ModuleID, hrStatus: HRESULT) -> HRESULT,
    pub ModuleAttachedToAssembly: unsafe extern "system" fn(
        this: &mut T,
        moduleId: ModuleID,
        AssemblyId: AssemblyID,
    ) -> HRESULT,
    pub ClassLoadStarted: unsafe extern "system" fn(this: &mut T, classId: ClassID) -> HRESULT,
    pub ClassLoadFinished:
        unsafe extern "system" fn(this: &mut T, classId: ClassID, hrStatus: HRESULT) -> HRESULT,
    pub ClassUnloadStarted: unsafe extern "system" fn(this: &mut T, classId: ClassID) -> HRESULT,
    pub ClassUnloadFinished:
        unsafe extern "system" fn(this: &mut T, classId: ClassID, hrStatus: HRESULT) -> HRESULT,
    pub FunctionUnloadStarted:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub JITCompilationStarted: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT,
    pub JITCompilationFinished: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        hrStatus: HRESULT,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT,
    pub JITCachedFunctionSearchStarted: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        pbUseCachedFunction: *mut BOOL,
    ) -> HRESULT,
    pub JITCachedFunctionSearchFinished: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> HRESULT,
    pub JITFunctionPitched:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub JITInlining: unsafe extern "system" fn(
        this: &mut T,
        callerId: FunctionID,
        calleeId: FunctionID,
        pfShouldInline: *mut BOOL,
    ) -> HRESULT,
    pub ThreadCreated: unsafe extern "system" fn(this: &mut T, threadId: ThreadID) -> HRESULT,
    pub ThreadDestroyed: unsafe extern "system" fn(this: &mut T, threadId: ThreadID) -> HRESULT,
    pub ThreadAssignedToOSThread: unsafe extern "system" fn(
        this: &mut T,
        managedThreadId: ThreadID,
        osThreadId: DWORD,
    ) -> HRESULT,
    pub RemotingClientInvocationStarted: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RemotingClientSendingMessage:
        unsafe extern "system" fn(this: &mut T, pCookie: *const GUID, fIsAsync: BOOL) -> HRESULT,
    pub RemotingClientReceivingReply:
        unsafe extern "system" fn(this: &mut T, pCookie: *const GUID, fIsAsync: BOOL) -> HRESULT,
    pub RemotingClientInvocationFinished: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RemotingServerReceivingMessage:
        unsafe extern "system" fn(this: &mut T, pCookie: *const GUID, fIsAsync: BOOL) -> HRESULT,
    pub RemotingServerInvocationStarted: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RemotingServerInvocationReturned: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RemotingServerSendingReply:
        unsafe extern "system" fn(this: &mut T, pCookie: *const GUID, fIsAsync: BOOL) -> HRESULT,
    pub UnmanagedToManagedTransition: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> HRESULT,
    pub ManagedToUnmanagedTransition: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> HRESULT,
    pub RuntimeSuspendStarted:
        unsafe extern "system" fn(this: &mut T, suspendReason: COR_PRF_SUSPEND_REASON) -> HRESULT,
    pub RuntimeSuspendFinished: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RuntimeSuspendAborted: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RuntimeResumeStarted: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RuntimeResumeFinished: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub RuntimeThreadSuspended:
        unsafe extern "system" fn(this: &mut T, threadId: ThreadID) -> HRESULT,
    pub RuntimeThreadResumed:
        unsafe extern "system" fn(this: &mut T, threadId: ThreadID) -> HRESULT,
    pub MovedReferences: unsafe extern "system" fn(
        this: &mut T,
        cMovedObjectIDRanges: ULONG,
        oldObjectIDRangeStart: *const ObjectID,
        newObjectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const ULONG,
    ) -> HRESULT,
    pub ObjectAllocated:
        unsafe extern "system" fn(this: &mut T, objectId: ObjectID, classId: ClassID) -> HRESULT,
    pub ObjectsAllocatedByClass: unsafe extern "system" fn(
        this: &mut T,
        cClassCount: ULONG,
        classIds: *const ClassID,
        cObjects: *const ULONG,
    ) -> HRESULT,
    pub ObjectReferences: unsafe extern "system" fn(
        this: &mut T,
        objectId: ObjectID,
        classId: ClassID,
        cObjectRefs: ULONG,
        objectRefIds: *const ObjectID,
    ) -> HRESULT,
    pub RootReferences: unsafe extern "system" fn(
        this: &mut T,
        cRootRefs: ULONG,
        rootRefIds: *const ObjectID,
    ) -> HRESULT,
    pub ExceptionThrown:
        unsafe extern "system" fn(this: &mut T, thrownObjectId: ObjectID) -> HRESULT,
    pub ExceptionSearchFunctionEnter:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub ExceptionSearchFunctionLeave: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ExceptionSearchFilterEnter:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub ExceptionSearchFilterLeave: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ExceptionSearchCatcherFound:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub ExceptionOSHandlerEnter:
        unsafe extern "system" fn(this: &mut T, __unused: UINT_PTR) -> HRESULT,
    pub ExceptionOSHandlerLeave:
        unsafe extern "system" fn(this: &mut T, __unused: UINT_PTR) -> HRESULT,
    pub ExceptionUnwindFunctionEnter:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub ExceptionUnwindFunctionLeave: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ExceptionUnwindFinallyEnter:
        unsafe extern "system" fn(this: &mut T, functionId: FunctionID) -> HRESULT,
    pub ExceptionUnwindFinallyLeave: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ExceptionCatcherEnter: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        objectId: ObjectID,
    ) -> HRESULT,
    pub ExceptionCatcherLeave: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub COMClassicVTableCreated: unsafe extern "system" fn(
        this: &mut T,
        wrappedClassId: ClassID,
        implementedIID: REFGUID,
        pVTable: *const c_void,
        cSlots: ULONG,
    ) -> HRESULT,
    pub COMClassicVTableDestroyed: unsafe extern "system" fn(
        this: &mut T,
        wrappedClassId: ClassID,
        implementedIID: REFGUID,
        pVTable: *const c_void,
    ) -> HRESULT,
    pub ExceptionCLRCatcherFound: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub ExceptionCLRCatcherExecute: unsafe extern "system" fn(this: &mut T) -> HRESULT,
}

impl ICorProfilerCallback<()> {
    // 176FBED1-A55C-4796-98CA-A9DA0EF883E7
    pub const IID: GUID = GUID {
        data1: 0x176FBED1,
        data2: 0xA55C,
        data3: 0x4796,
        data4: [0x98, 0xCA, 0xA9, 0xDA, 0x0E, 0xF8, 0x83, 0xE7],
    };
}
