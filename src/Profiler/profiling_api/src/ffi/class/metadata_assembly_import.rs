#![allow(non_snake_case)]
use crate::ffi::{
    mdAssembly, mdAssemblyRef, mdExportedType, mdFile, mdManifestResource, mdToken, mdTypeDef,
    IMetaDataAssemblyImport, IUnknown, ASSEMBLYMETADATA, DWORD, HCORENUM, HRESULT, LPCWSTR, ULONG,
    WCHAR,
};
use std::ffi::c_void;

#[repr(C)]
pub struct MetaDataAssemblyImportVtbl {
    pub IUnknown: IUnknown<MetaDataAssemblyImport>,
    pub IMetaDataAssemblyImport: IMetaDataAssemblyImport<MetaDataAssemblyImport>,
}

#[repr(C)]
pub struct MetaDataAssemblyImport {
    pub lpVtbl: *const MetaDataAssemblyImportVtbl,
}

impl MetaDataAssemblyImport {
    pub unsafe fn i_metadata_assembly_import(&self) -> &IMetaDataAssemblyImport<Self> {
        &(*self.lpVtbl).IMetaDataAssemblyImport
    }
    pub unsafe fn GetAssemblyProps(
        &self,
        mda: mdAssembly,
        ppbPublicKey: *mut *mut c_void,
        pcbPublicKey: *mut ULONG,
        pulHashAlgId: *mut ULONG,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pMetaData: *mut ASSEMBLYMETADATA,
        pdwAssemblyFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().GetAssemblyProps)(
            self,
            mda,
            ppbPublicKey,
            pcbPublicKey,
            pulHashAlgId,
            szName,
            cchName,
            pchName,
            pMetaData,
            pdwAssemblyFlags,
        )
    }
    pub unsafe fn GetAssemblyRefProps(
        &self,
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
    ) -> HRESULT {
        (self.i_metadata_assembly_import().GetAssemblyRefProps)(
            self,
            mdar,
            ppbPublicKeyOrToken,
            pcbPublicKeyOrToken,
            szName,
            cchName,
            pchName,
            pMetaData,
            ppbHashValue,
            pcbHashValue,
            pdwAssemblyRefFlags,
        )
    }
    pub unsafe fn GetFileProps(
        &self,
        mdf: mdFile,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ppbHashValue: *mut *mut c_void,
        pcbHashValue: *mut ULONG,
        pdwFileFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().GetFileProps)(
            self,
            mdf,
            szName,
            cchName,
            pchName,
            ppbHashValue,
            pcbHashValue,
            pdwFileFlags,
        )
    }
    pub unsafe fn GetExportedTypeProps(
        &self,
        mdct: mdExportedType,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ptkImplementation: *mut mdToken,
        ptkTypeDef: *mut mdTypeDef,
        pdwExportedTypeFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().GetExportedTypeProps)(
            self,
            mdct,
            szName,
            cchName,
            pchName,
            ptkImplementation,
            ptkTypeDef,
            pdwExportedTypeFlags,
        )
    }
    pub unsafe fn GetManifestResourceProps(
        &self,
        mdmr: mdManifestResource,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        ptkImplementation: *mut mdToken,
        pdwOffset: *mut DWORD,
        pdwResourceFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().GetManifestResourceProps)(
            self,
            mdmr,
            szName,
            cchName,
            pchName,
            ptkImplementation,
            pdwOffset,
            pdwResourceFlags,
        )
    }
    pub unsafe fn EnumAssemblyRefs(
        &self,
        phEnum: *mut HCORENUM,
        rAssemblyRefs: *mut mdAssemblyRef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().EnumAssemblyRefs)(
            self,
            phEnum,
            rAssemblyRefs,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn EnumFiles(
        &self,
        phEnum: *mut HCORENUM,
        rFiles: *mut mdFile,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().EnumFiles)(self, phEnum, rFiles, cMax, pcTokens)
    }
    pub unsafe fn EnumExportedTypes(
        &self,
        phEnum: *mut HCORENUM,
        rExportedTypes: *mut mdExportedType,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().EnumExportedTypes)(
            self,
            phEnum,
            rExportedTypes,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn EnumManifestResources(
        &self,
        phEnum: *mut HCORENUM,
        rManifestResources: *mut mdManifestResource,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().EnumManifestResources)(
            self,
            phEnum,
            rManifestResources,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn GetAssemblyFromScope(&self, ptkAssembly: *mut mdAssembly) -> HRESULT {
        (self.i_metadata_assembly_import().GetAssemblyFromScope)(self, ptkAssembly)
    }
    pub unsafe fn FindExportedTypeByName(
        &self,
        szName: LPCWSTR,
        mdtExportedType: mdToken,
        ptkExportedType: *mut mdExportedType,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().FindExportedTypeByName)(
            self,
            szName,
            mdtExportedType,
            ptkExportedType,
        )
    }
    pub unsafe fn FindManifestResourceByName(
        &self,
        szName: LPCWSTR,
        ptkManifestResource: *mut mdManifestResource,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().FindManifestResourceByName)(
            self,
            szName,
            ptkManifestResource,
        )
    }
    pub unsafe fn CloseEnum(&self, hEnum: HCORENUM) -> () {
        (self.i_metadata_assembly_import().CloseEnum)(self, hEnum)
    }
    pub unsafe fn FindAssembliesByName(
        &self,
        szAppBase: LPCWSTR,
        szPrivateBin: LPCWSTR,
        szAssemblyName: LPCWSTR,
        ppIUnk: *mut *mut Self,
        cMax: ULONG,
        pcAssemblies: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_assembly_import().FindAssembliesByName)(
            self,
            szAppBase,
            szPrivateBin,
            szAssemblyName,
            ppIUnk,
            cMax,
            pcAssemblies,
        )
    }
}
