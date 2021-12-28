#![allow(non_snake_case)]
use crate::ffi::{
    mdAssembly, mdAssemblyRef, mdExportedType, mdFile, mdManifestResource, mdToken, mdTypeDef,
    IMetaDataAssemblyEmit, IUnknown, ASSEMBLYMETADATA, DWORD, HRESULT, LPCWSTR, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct MetaDataAssemblyEmitVtbl {
    pub IUnknown: IUnknown<MetaDataAssemblyEmit>,
    pub IMetaDataAssemblyEmit: IMetaDataAssemblyEmit<MetaDataAssemblyEmit>,
}

#[repr(C)]
pub struct MetaDataAssemblyEmit {
    pub lpVtbl: *const MetaDataAssemblyEmitVtbl,
}

impl MetaDataAssemblyEmit {
    pub unsafe fn i_metadata_assembly_emit(&self) -> &IMetaDataAssemblyEmit<Self> {
        &(*self.lpVtbl).IMetaDataAssemblyEmit
    }
    pub unsafe fn DefineAssembly(
        &self,
        pbPublicKey: *const c_void,
        cbPublicKey: ULONG,
        ulHashAlgId: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        dwAssemblyFlags: DWORD,
        pmda: *mut mdAssembly,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().DefineAssembly)(
            self,
            pbPublicKey,
            cbPublicKey,
            ulHashAlgId,
            szName,
            pMetaData,
            dwAssemblyFlags,
            pmda,
        )
    }
    pub unsafe fn DefineAssemblyRef(
        &self,
        pbPublicKeyOrToken: *const c_void,
        cbPublicKeyOrToken: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwAssemblyRefFlags: DWORD,
        pmdar: *mut mdAssemblyRef,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().DefineAssemblyRef)(
            self,
            pbPublicKeyOrToken,
            cbPublicKeyOrToken,
            szName,
            pMetaData,
            pbHashValue,
            cbHashValue,
            dwAssemblyRefFlags,
            pmdar,
        )
    }
    pub unsafe fn DefineExportedType(
        &self,
        szName: LPCWSTR,
        tkImplementation: mdToken,
        tkTypeDef: mdTypeDef,
        dwExportedTypeFlags: DWORD,
        pmdct: *mut mdExportedType,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().DefineExportedType)(
            self,
            szName,
            tkImplementation,
            tkTypeDef,
            dwExportedTypeFlags,
            pmdct,
        )
    }
    pub unsafe fn DefineManifestResource(
        &self,
        szName: LPCWSTR,
        tkImplementation: mdToken,
        dwOffset: DWORD,
        dwResourceFlags: DWORD,
        pmdmr: *mut mdManifestResource,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().DefineManifestResource)(
            self,
            szName,
            tkImplementation,
            dwOffset,
            dwResourceFlags,
            pmdmr,
        )
    }
    pub unsafe fn SetAssemblyProps(
        &self,
        pma: mdAssembly,
        pbPublicKey: *const c_void,
        cbPublicKey: ULONG,
        ulHashAlgId: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        dwAssemblyFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().SetAssemblyProps)(
            self,
            pma,
            pbPublicKey,
            cbPublicKey,
            ulHashAlgId,
            szName,
            pMetaData,
            dwAssemblyFlags,
        )
    }
    pub unsafe fn SetAssemblyRefProps(
        &self,
        ar: mdAssemblyRef,
        pbPublicKeyOrToken: *const c_void,
        cbPublicKeyOrToken: ULONG,
        szName: LPCWSTR,
        pMetaData: *const ASSEMBLYMETADATA,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwAssemblyRefFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().SetAssemblyRefProps)(
            self,
            ar,
            pbPublicKeyOrToken,
            cbPublicKeyOrToken,
            szName,
            pMetaData,
            pbHashValue,
            cbHashValue,
            dwAssemblyRefFlags,
        )
    }
    pub unsafe fn SetFileProps(
        &self,
        file: mdFile,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        dwFileFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().SetFileProps)(
            self,
            file,
            pbHashValue,
            cbHashValue,
            dwFileFlags,
        )
    }
    pub unsafe fn SetExportedTypeProps(
        &self,
        ct: mdExportedType,
        tkImplementation: mdToken,
        tkTypeDef: mdTypeDef,
        dwExportedTypeFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().SetExportedTypeProps)(
            self,
            ct,
            tkImplementation,
            tkTypeDef,
            dwExportedTypeFlags,
        )
    }
    pub unsafe fn SetManifestResourceProps(
        &self,
        mr: mdManifestResource,
        tkImplementation: mdToken,
        dwOffset: DWORD,
        dwResourceFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_assembly_emit().SetManifestResourceProps)(
            self,
            mr,
            tkImplementation,
            dwOffset,
            dwResourceFlags,
        )
    }
}
