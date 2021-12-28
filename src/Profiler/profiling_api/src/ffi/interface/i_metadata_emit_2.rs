#![allow(non_snake_case)]
use crate::ffi::{
    mdGenericParam, mdMethodSpec, mdToken, CorSaveSize, Unknown, DWORD, GUID, HRESULT, LPCWSTR,
    PCCOR_SIGNATURE, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct IMetaDataEmit2<T> {
    pub DefineMethodSpec: unsafe extern "system" fn(
        this: &T,
        tkParent: mdToken,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmi: *mut mdMethodSpec,
    ) -> HRESULT,
    pub GetDeltaSaveSize:
        unsafe extern "system" fn(this: &T, fSave: CorSaveSize, pdwSaveSize: *mut DWORD) -> HRESULT,
    pub SaveDelta:
        unsafe extern "system" fn(this: &T, szFile: LPCWSTR, dwSaveFlags: DWORD) -> HRESULT,
    pub SaveDeltaToStream: unsafe extern "system" fn(
        this: &T,
        pIStream: *const Unknown, // TODO: Implement ISequentialStream, IStream and then Stream co-class
        dwSaveFlags: DWORD,
    ) -> HRESULT,
    pub SaveDeltaToMemory:
        unsafe extern "system" fn(this: &T, pbData: *mut c_void, cbData: ULONG) -> HRESULT,
    pub DefineGenericParam: unsafe extern "system" fn(
        this: &T,
        tk: mdToken,
        ulParamSeq: ULONG,
        dwParamFlags: DWORD,
        szname: LPCWSTR,
        reserved: DWORD,
        rtkConstraints: *const mdToken,
        pgp: *mut mdGenericParam,
    ) -> HRESULT,
    pub SetGenericParamProps: unsafe extern "system" fn(
        this: &T,
        gp: mdGenericParam,
        dwParamFlags: DWORD,
        szName: LPCWSTR,
        reserved: DWORD,
        rtkConstraints: *const mdToken,
    ) -> HRESULT,
    pub ResetENCLog: unsafe extern "system" fn(this: &T) -> HRESULT,
}

impl IMetaDataEmit2<()> {
    // F5DD9950-F693-42E6-830E-7B833E8146A9
    pub const IID: GUID = GUID {
        data1: 0xF5DD9950,
        data2: 0xF693,
        data3: 0x42E6,
        data4: [0x83, 0x0E, 0x7B, 0x83, 0x3E, 0x81, 0x46, 0xA9],
    };
}
