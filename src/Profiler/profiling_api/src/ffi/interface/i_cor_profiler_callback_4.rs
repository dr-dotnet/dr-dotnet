#![allow(non_snake_case)]
use crate::ffi::{
    mdMethodDef, CorProfilerFunctionControl, FunctionID, ModuleID, ObjectID, ReJITID, BOOL, GUID,
    HRESULT, SIZE_T, ULONG,
};

#[repr(C)]
pub struct ICorProfilerCallback4<T> {
    pub ReJITCompilationStarted: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        rejitId: ReJITID,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT,
    pub GetReJITParameters: unsafe extern "system" fn(
        this: &mut T,
        moduleId: ModuleID,
        methodId: mdMethodDef,
        pFunctionControl: *const CorProfilerFunctionControl,
    ) -> HRESULT,
    pub ReJITCompilationFinished: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        rejitId: ReJITID,
        hrStatus: HRESULT,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT,
    pub ReJITError: unsafe extern "system" fn(
        this: &mut T,
        moduleId: ModuleID,
        methodId: mdMethodDef,
        functionId: FunctionID,
        hrStatus: HRESULT,
    ) -> HRESULT,
    pub MovedReferences2: unsafe extern "system" fn(
        this: &mut T,
        cMovedObjectIDRanges: ULONG,
        oldObjectIDRangeStart: *const ObjectID,
        newObjectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const SIZE_T,
    ) -> HRESULT,
    pub SurvivingReferences2: unsafe extern "system" fn(
        this: &mut T,
        cSurvivingObjectIDRanges: ULONG,
        objectIDRangeStart: *const ObjectID,
        cObjectIDRangeLength: *const SIZE_T,
    ) -> HRESULT,
}

impl ICorProfilerCallback4<()> {
    // 7B63B2E3-107D-4D48-B2F6-F61E229470D2
    pub const IID: GUID = GUID {
        data1: 0x7B63B2E3,
        data2: 0x107D,
        data3: 0x4D48,
        data4: [0xB2, 0xF6, 0xF6, 0x1E, 0x22, 0x94, 0x70, 0xD2],
    };
}
