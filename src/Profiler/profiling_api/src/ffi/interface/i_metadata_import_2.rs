#![allow(non_snake_case)]
use crate::ffi::{
    mdGenericParam, mdGenericParamConstraint, mdMethodSpec, mdToken, DWORD, GUID, HCORENUM,
    HRESULT, PCCOR_SIGNATURE, ULONG, WCHAR,
};

#[repr(C)]
pub struct IMetaDataImport2<T> {
    pub EnumGenericParams: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        rGenericParams: *mut mdGenericParam,
        cMax: ULONG,
        pcGenericParams: *mut ULONG,
    ) -> HRESULT,
    pub GetGenericParamProps: unsafe extern "system" fn(
        this: &T,
        gp: mdGenericParam,
        pulParamSeq: *mut ULONG,
        pdwParamFlags: *mut DWORD,
        ptOwner: *mut mdToken,
        reserved: *mut DWORD,
        wzname: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT,
    pub GetMethodSpecProps: unsafe extern "system" fn(
        this: &T,
        mi: mdMethodSpec,
        tkParent: *mut mdToken,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
    ) -> HRESULT,
    pub EnumGenericParamConstraints: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tk: mdGenericParam,
        rGenericParamConstraints: *mut mdGenericParamConstraint,
        cMax: ULONG,
        pcGenericParamConstraints: *mut ULONG,
    ) -> HRESULT,
    pub GetGenericParamConstraintProps: unsafe extern "system" fn(
        this: &T,
        gpc: mdGenericParamConstraint,
        ptGenericParam: *mut mdGenericParam,
        ptkConstraintType: *mut mdToken,
    ) -> HRESULT,
    pub GetPEKind: unsafe extern "system" fn(
        this: &T,
        pdwPEKind: *mut DWORD,
        pdwMAchine: *mut DWORD,
    ) -> HRESULT,
    pub GetVersionString: unsafe extern "system" fn(
        this: &T,
        pwzBuf: *mut WCHAR,
        ccBufSize: DWORD,
        pccBufSize: *mut DWORD,
    ) -> HRESULT,
    pub EnumMethodSpecs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        rMethodSpecs: *mut mdMethodSpec,
        cMax: ULONG,
        pcMethodSpecs: *mut ULONG,
    ) -> HRESULT,
}

impl IMetaDataImport2<()> {
    // FCE5EFA0-8BBA-4F8E-A036-8F2022B08466
    pub const IID: GUID = GUID {
        data1: 0xFCE5EFA0,
        data2: 0x8BBA,
        data3: 0x4F8E,
        data4: [0xA0, 0x36, 0x8F, 0x20, 0x22, 0xB0, 0x84, 0x66],
    };
}
