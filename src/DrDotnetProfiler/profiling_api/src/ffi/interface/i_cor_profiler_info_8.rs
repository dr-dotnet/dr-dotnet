#![allow(non_snake_case)]
use crate::ffi::{
    FunctionID, ModuleID, ReJITID, BOOL, GUID, HRESULT, LPCBYTE, PCCOR_SIGNATURE, ULONG, WCHAR,
};

#[repr(C)]
pub struct ICorProfilerInfo8<T> {
    pub IsFunctionDynamic: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        isDynamic: *mut BOOL,
    ) -> HRESULT,
    pub GetFunctionFromIP3: unsafe extern "system" fn(
        this: &T,
        ip: LPCBYTE,
        functionId: *mut FunctionID,
        pReJitId: *mut ReJITID,
    ) -> HRESULT,
    pub GetDynamicFunctionInfo: unsafe extern "system" fn(
        this: &T,
        functionId: FunctionID,
        moduleId: *mut ModuleID,
        ppvSig: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
        cchName: ULONG,
        pcchName: *mut ULONG,
        wszName: *mut WCHAR,
    ) -> HRESULT,
}

impl ICorProfilerInfo8<()> {
    // C5AC80A6-782E-4716-8044-39598C60CFBF
    pub const IID: GUID = GUID {
        data1: 0xC5AC80A6,
        data2: 0x782E,
        data3: 0x4716,
        data4: [0x80, 0x44, 0x39, 0x59, 0x8C, 0x60, 0xCF, 0xBF],
    };
}
