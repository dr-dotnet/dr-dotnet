#![allow(non_snake_case)]
use crate::ffi::{
    mdMethodDef, ModuleID, ObjectID, ObjectReferenceCallback, BOOL, DWORD, GUID, HRESULT, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct ICorProfilerInfo10<T> {
    pub EnumerateObjectReferences: unsafe extern "system" fn(
        this: &T,
        objectId: ObjectID,
        callback: ObjectReferenceCallback,
        clientData: *const c_void,
    ) -> HRESULT,
    pub IsFrozenObject:
        unsafe extern "system" fn(this: &T, objectId: ObjectID, pbFrozen: *mut BOOL) -> HRESULT,
    pub GetLOHObjectSizeThreshold:
        unsafe extern "system" fn(this: &T, pThreshold: *mut DWORD) -> HRESULT,
    pub RequestReJITWithInliners: unsafe extern "system" fn(
        this: &T,
        dwRejitFlags: DWORD,
        cFunctions: ULONG,
        moduleIds: *const ModuleID,
        methodIds: *const mdMethodDef,
    ) -> HRESULT,
    pub SuspendRuntime: unsafe extern "system" fn(this: &T) -> HRESULT,
    pub ResumeRuntime: unsafe extern "system" fn(this: &T) -> HRESULT,
}

impl ICorProfilerInfo10<()> {
    // 2F1B5152-C869-40C9-AA5F-3ABE026BD720
    pub const IID: GUID = GUID {
        data1: 0x2F1B5152,
        data2: 0xC869,
        data3: 0x40C9,
        data4: [0xAA, 0x5F, 0x3A, 0xBE, 0x02, 0x6B, 0xD7, 0x20],
    };
}
