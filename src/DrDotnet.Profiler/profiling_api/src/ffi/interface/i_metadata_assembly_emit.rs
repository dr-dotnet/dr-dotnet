#![allow(non_snake_case)]
use crate::ffi::{
    mdAssembly, mdAssemblyRef, mdExportedType, mdFile, mdManifestResource, mdToken, mdTypeDef,
    ASSEMBLYMETADATA, DWORD, GUID, HRESULT, LPCWSTR, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct IMetaDataAssemblyEmit<T> {
    pub DefineAssembly: unsafe extern "system" fn(
        this: &T,
        pbPublicKey: *const c_void,
        cbPublicKey: ULONG,
        ulHashAlgId: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        dwAssemblyFlags: DWORD,
        pmda: *mut mdAssembly,
    ) -> HRESULT,
    pub DefineAssemblyRef: unsafe extern "system" fn(
        this: &T,
        pbPublicKeyOrToken: *const c_void,
        cbPublicKeyOrToken: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwAssemblyRefFlags: DWORD,
        pmdar: *mut mdAssemblyRef,
    ) -> HRESULT,
    pub DefineExportedType: unsafe extern "system" fn(
        this: &T,
        szName: LPCWSTR,
        tkImplementation: mdToken,
        tkTypeDef: mdTypeDef,
        dwExportedTypeFlags: DWORD,
        pmdct: *mut mdExportedType,
    ) -> HRESULT,
    pub DefineManifestResource: unsafe extern "system" fn(
        this: &T,
        szName: LPCWSTR,
        tkImplementation: mdToken,
        dwOffset: DWORD,
        dwResourceFlags: DWORD,
        pmdmr: *mut mdManifestResource,
    ) -> HRESULT,
    pub SetAssemblyProps: unsafe extern "system" fn(
        this: &T,
        pma: mdAssembly,
        pbPublicKey: *const c_void,
        cbPublicKey: ULONG,
        ulHashAlgId: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        dwAssemblyFlags: DWORD,
    ) -> HRESULT,
    pub SetAssemblyRefProps: unsafe extern "system" fn(
        this: &T,
        ar: mdAssemblyRef,
        pbPublicKeyOrToken: *const c_void,
        cbPublicKeyOrToken: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwAssemblyRefFlags: DWORD,
    ) -> HRESULT,
    pub SetFileProps: unsafe extern "system" fn(
        this: &T,
        file: mdFile,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwFileFlags: DWORD,
    ) -> HRESULT,
    pub SetExportedTypeProps: unsafe extern "system" fn(
        this: &T,
        ct: mdExportedType,
        tkImplementation: mdToken,
        tkTypeDef: mdTypeDef,
        dwExportedTypeFlags: DWORD,
    ) -> HRESULT,
    pub SetManifestResourceProps: unsafe extern "system" fn(
        this: &T,
        mr: mdManifestResource,
        tkImplementation: mdToken,
        dwOffset: DWORD,
        dwResourceFlags: DWORD,
    ) -> HRESULT,
}

impl IMetaDataAssemblyEmit<()> {
    // 211EF15B-5317-4438-B196-DEC87B887693
    pub const IID: GUID = GUID {
        data1: 0x211EF15B,
        data2: 0x5317,
        data3: 0x4438,
        data4: [0xB1, 0x96, 0xDE, 0xC8, 0x7B, 0x88, 0x76, 0x93],
    };
}
