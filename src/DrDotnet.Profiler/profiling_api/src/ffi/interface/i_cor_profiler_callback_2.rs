#![allow(non_snake_case)]
use crate::ffi::{
    int, GCHandleID, ObjectID, ThreadID, BOOL, COR_PRF_GC_REASON, COR_PRF_GC_ROOT_FLAGS,
    COR_PRF_GC_ROOT_KIND, DWORD, GUID, HRESULT, UINT_PTR, ULONG, WCHAR,
};

#[repr(C)]
pub struct ICorProfilerCallback2<T> {
    pub ThreadNameChanged: unsafe extern "system" fn(
        this: &mut T,
        threadId: ThreadID,
        cchName: ULONG,
        name: *const WCHAR,
    ) -> HRESULT,
    pub GarbageCollectionStarted: unsafe extern "system" fn(
        this: &mut T,
        cGenerations: int,
        generationCollected: *const BOOL,
        reason: COR_PRF_GC_REASON,
    ) -> HRESULT,
    pub SurvivingReferences: unsafe extern "system" fn(
        this: &mut T,
        cSurvivingObjectIDRanges: ULONG,
        objectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const ULONG,
    ) -> HRESULT,
    pub GarbageCollectionFinished: unsafe extern "system" fn(this: &mut T) -> HRESULT,
    pub FinalizeableObjectQueued: unsafe extern "system" fn(
        this: &mut T,
        finalizerFlags: DWORD,
        objectID: ObjectID,
    ) -> HRESULT,
    pub RootReferences2: unsafe extern "system" fn(
        this: &mut T,
        cRootRefs: ULONG,
        rootRefIds: *const ObjectID,
        rootKinds: *const COR_PRF_GC_ROOT_KIND,
        rootFlags: *const COR_PRF_GC_ROOT_FLAGS,
        rootIds: *const UINT_PTR,
    ) -> HRESULT,
    pub HandleCreated: unsafe extern "system" fn(
        this: &mut T,
        handleId: GCHandleID,
        initialObjectId: ObjectID,
    ) -> HRESULT,
    pub HandleDestroyed: unsafe extern "system" fn(this: &mut T, handleId: GCHandleID) -> HRESULT,
}

impl ICorProfilerCallback2<()> {
    // 8A8CC829-CCF2-49FE-BBAE-0F022228071A
    pub const IID: GUID = GUID {
        data1: 0x8A8CC829,
        data2: 0xCCF2,
        data3: 0x49FE,
        data4: [0xBB, 0xAE, 0x0F, 0x02, 0x22, 0x28, 0x07, 0x1A],
    };
}
