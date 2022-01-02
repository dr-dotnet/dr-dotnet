#![allow(non_snake_case)]
use crate::ffi::{FunctionID, BOOL, GUID, HRESULT, LPCBYTE, ULONG};

#[repr(C)]
pub struct ICorProfilerCallback8<T> {
    pub DynamicMethodJITCompilationStarted: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        fIsSafeToBlock: BOOL,
        pILHeader: LPCBYTE,
        cbILHeader: ULONG,
    ) -> HRESULT,
    pub DynamicMethodJITCompilationFinished: unsafe extern "system" fn(
        this: &mut T,
        functionId: FunctionID,
        hrStatus: HRESULT,
        fIsSafeToBlock: BOOL,
    ) -> HRESULT,
}

impl ICorProfilerCallback8<()> {
    // 5BED9B15-C079-4D47-BFE2-215A140C07E0
    pub const IID: GUID = GUID {
        data1: 0x5BED9B15,
        data2: 0xC079,
        data3: 0x4D47,
        data4: [0xBF, 0xE2, 0x21, 0x5A, 0x14, 0x0C, 0x07, 0xE0],
    };
}
