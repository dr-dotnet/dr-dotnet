#![allow(non_snake_case)]
use crate::ffi::{
    mdAssembly, mdAssemblyRef, mdExportedType, mdFile, mdManifestResource, mdToken, mdTypeDef,
    ASSEMBLYMETADATA, DWORD, GUID, HCORENUM, HRESULT, LPCWSTR, ULONG, WCHAR,
};
use std::ffi::c_void;

#[repr(C)]
pub struct IMetaDataAssemblyImport<T> {
    pub GetAssemblyProps: unsafe extern "system" fn(
        this: &T,
        mda: mdAssembly,
        ppbPublicKey: *mut *mut c_void,
        pcbPublicKey: *mut ULONG,
        pulHashAlgId: *mut ULONG,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pMetaData: *mut ASSEMBLYMETADATA,
        pdwAssemblyFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetAssemblyRefProps: unsafe extern "system" fn(
        this: &T,
        mdar: mdAssemblyRef,
        ppbPublicKeyOrToken: *mut *mut c_void,
        pcbPublicKeyOrToken: *mut ULONG,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pMetaData: *mut ASSEMBLYMETADATA,
        ppbHashValue: *mut *mut c_void,
        pcbHashValue: *mut ULONG,
        pdwAssemblyRefFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetFileProps: unsafe extern "system" fn(
        this: &T,
        mdf: mdFile,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ppbHashValue: *mut *mut c_void,
        pcbHashValue: *mut ULONG,
        pdwFileFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetExportedTypeProps: unsafe extern "system" fn(
        this: &T,
        mdct: mdExportedType,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ptkImplementation: *mut mdToken,
        ptkTypeDef: *mut mdTypeDef,
        pdwExportedTypeFlags: *mut DWORD,
    ) -> HRESULT,
    pub GetManifestResourceProps: unsafe extern "system" fn(
        this: &T,
        mdmr: mdManifestResource,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ptkImplementation: *mut mdToken,
        pdwOffset: *mut DWORD,
        pdwResourceFlags: *mut DWORD,
    ) -> HRESULT,
    pub EnumAssemblyRefs: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rAssemblyRefs: *mut mdAssemblyRef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumFiles: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rFiles: *mut mdFile,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumExportedTypes: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rExportedTypes: *mut mdExportedType,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub EnumManifestResources: unsafe extern "system" fn(
        this: &T,
        phEnum: *mut HCORENUM,
        rManifestResources: *mut mdManifestResource,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT,
    pub GetAssemblyFromScope:
        unsafe extern "system" fn(this: &T, ptkAssembly: *mut mdAssembly) -> HRESULT,
    pub FindExportedTypeByName: unsafe extern "system" fn(
        this: &T,
        szName: LPCWSTR,
        mdtExportedType: mdToken,
        ptkExportedType: *mut mdExportedType,
    ) -> HRESULT,
    pub FindManifestResourceByName: unsafe extern "system" fn(
        this: &T,
        szName: LPCWSTR,
        ptkManifestResource: *mut mdManifestResource,
    ) -> HRESULT,
    pub CloseEnum: unsafe extern "system" fn(this: &T, hEnum: HCORENUM) -> (),
    pub FindAssembliesByName: unsafe extern "system" fn(
        this: &T,
        szAppBase: LPCWSTR,
        szPrivateBin: LPCWSTR,
        szAssemblyName: LPCWSTR,
        ppIUnk: *mut *mut T,
        cMax: ULONG,
        pcAssemblies: *mut ULONG,
    ) -> HRESULT,
}

impl IMetaDataAssemblyImport<()> {
    // EE62470B-E94B-424E-9B7C-2F00C9249F93
    pub const IID: GUID = GUID {
        data1: 0xEE62470B,
        data2: 0xE94B,
        data3: 0x424E,
        data4: [0x9B, 0x7C, 0x2F, 0x00, 0xC9, 0x24, 0x9F, 0x93],
    };
}
