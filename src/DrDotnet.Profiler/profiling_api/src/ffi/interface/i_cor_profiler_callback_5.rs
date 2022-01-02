#![allow(non_snake_case)]
use crate::ffi::{GCHandleID, ObjectID, GUID, HRESULT, ULONG};

#[repr(C)]
pub struct ICorProfilerCallback5<T> {
    pub ConditionalWeakTableElementReferences: unsafe extern "system" fn(
        this: &mut T,
        cRootRefs: ULONG,
        keyRefIds: *const ObjectID,
        valueRefIds: *const ObjectID,
        rootIds: *const GCHandleID,
    ) -> HRESULT,
}

impl ICorProfilerCallback5<()> {
    // 8DFBA405-8C9F-45F8-BFFA-83B14CEF78B5
    pub const IID: GUID = GUID {
        data1: 0x8DFBA405,
        data2: 0x8C9F,
        data3: 0x45F8,
        data4: [0xBF, 0xFA, 0x83, 0xB1, 0x4C, 0xEF, 0x78, 0xB5],
    };
}
