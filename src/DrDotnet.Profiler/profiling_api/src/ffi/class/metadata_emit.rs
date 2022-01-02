#![allow(non_snake_case)]
use super::{MetaDataAssemblyEmit, MetaDataAssemblyImport, MetaDataImport, Unknown};
use crate::ffi::{
    mdCustomAttribute, mdEvent, mdFieldDef, mdGenericParam, mdMemberRef, mdMethodDef, mdMethodSpec,
    mdModuleRef, mdParamDef, mdPermission, mdProperty, mdSignature, mdString, mdToken, mdTypeDef,
    mdTypeRef, mdTypeSpec, CorSaveSize, IMetaDataEmit, IMetaDataEmit2, IUnknown, COR_FIELD_OFFSET,
    COR_SECATTR, DWORD, HRESULT, LPCWSTR, PCCOR_SIGNATURE, PCOR_SIGNATURE, ULONG,
};
use std::ffi::c_void;

#[repr(C)]
pub struct MetaDataEmitVtbl {
    pub IUnknown: IUnknown<MetaDataEmit>,
    pub IMetaDataEmit: IMetaDataEmit<MetaDataEmit>,
    pub IMetaDataEmit2: IMetaDataEmit2<MetaDataEmit>,
}

#[repr(C)]
pub struct MetaDataEmit {
    pub lpVtbl: *const MetaDataEmitVtbl,
}

impl MetaDataEmit {
    pub unsafe fn i_metadata_emit(&self) -> &IMetaDataEmit<Self> {
        &(*self.lpVtbl).IMetaDataEmit
    }
    pub unsafe fn i_metadata_emit_2(&self) -> &IMetaDataEmit2<Self> {
        &(*self.lpVtbl).IMetaDataEmit2
    }
    pub unsafe fn SetModuleProps(&self, szName: LPCWSTR) -> HRESULT {
        (self.i_metadata_emit().SetModuleProps)(self, szName)
    }
    pub unsafe fn Save(&self, szName: LPCWSTR, dwSaveFlags: DWORD) -> HRESULT {
        (self.i_metadata_emit().Save)(self, szName, dwSaveFlags)
    }
    pub unsafe fn SaveToStream(
        &self,
        pIStream: *const Unknown, // TODO: Implement ISequentialStream, IStream and then Stream co-class
        dwSaveFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_emit().SaveToStream)(self, pIStream, dwSaveFlags)
    }
    pub unsafe fn GetSaveSize(&self, fSave: CorSaveSize, pdwSaveSize: *mut DWORD) -> HRESULT {
        (self.i_metadata_emit().GetSaveSize)(self, fSave, pdwSaveSize)
    }
    pub unsafe fn DefineTypeDef(
        &self,
        szTypeDef: LPCWSTR,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
        ptd: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineTypeDef)(
            self,
            szTypeDef,
            dwTypeDefFlags,
            tkExtends,
            rtkImplements,
            ptd,
        )
    }
    pub unsafe fn DefineNestedType(
        &self,
        szTypeDef: LPCWSTR,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
        tdEncloser: mdTypeDef,
        ptd: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineNestedType)(
            self,
            szTypeDef,
            dwTypeDefFlags,
            tkExtends,
            rtkImplements,
            tdEncloser,
            ptd,
        )
    }
    pub unsafe fn SetHandler(&self, pUnk: *const Unknown) -> HRESULT {
        (self.i_metadata_emit().SetHandler)(self, pUnk)
    }
    pub unsafe fn DefineMethod(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        dwMethodFlags: DWORD,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        ulCodeRVA: ULONG,
        dwImplFlags: DWORD,
        pmd: *const mdMethodDef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineMethod)(
            self,
            td,
            szName,
            dwMethodFlags,
            pvSigBlob,
            cbSigBlob,
            ulCodeRVA,
            dwImplFlags,
            pmd,
        )
    }
    pub unsafe fn DefineMethodImpl(
        &self,
        td: mdTypeDef,
        tkBody: mdToken,
        tkDecl: mdToken,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineMethodImpl)(self, td, tkBody, tkDecl)
    }
    pub unsafe fn DefineTypeRefByName(
        &self,
        tkResolutionScope: mdToken,
        szName: LPCWSTR,
        ptr: *mut mdTypeRef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineTypeRefByName)(self, tkResolutionScope, szName, ptr)
    }
    pub unsafe fn DefineImportType(
        &self,
        pAssemImport: *const MetaDataAssemblyImport,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        pImport: *const MetaDataImport,
        tdImport: mdTypeDef,
        pAssemEmit: *const MetaDataAssemblyEmit,
        ptr: *mut mdTypeRef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineImportType)(
            self,
            pAssemImport,
            pbHashValue,
            cbHashValue,
            pImport,
            tdImport,
            pAssemEmit,
            ptr,
        )
    }
    pub unsafe fn DefineMemberRef(
        &self,
        tkImport: mdToken,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmr: *mut mdMemberRef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineMemberRef)(self, tkImport, szName, pvSigBlob, cbSigBlob, pmr)
    }
    pub unsafe fn DefineImportMember(
        &self,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        pImport: *const MetaDataImport,
        mbMember: mdToken,
        pAssemEmit: *const MetaDataAssemblyEmit,
        tkParent: mdToken,
        pmr: *mut mdMemberRef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineImportMember)(
            self,
            pbHashValue,
            cbHashValue,
            pImport,
            mbMember,
            pAssemEmit,
            tkParent,
            pmr,
        )
    }
    pub unsafe fn DefineEvent(
        &self,
        td: mdTypeDef,
        szEvent: LPCWSTR,
        dwEventFlags: DWORD,
        tkEventType: mdToken,
        mdAddOn: mdMethodDef,
        mdRemoveOn: mdMethodDef,
        mdFire: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
        pmdEvent: *mut mdEvent,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineEvent)(
            self,
            td,
            szEvent,
            dwEventFlags,
            tkEventType,
            mdAddOn,
            mdRemoveOn,
            mdFire,
            rmdOtherMethods,
            pmdEvent,
        )
    }
    pub unsafe fn SetClassLayout(
        &self,
        td: mdTypeDef,
        dwPackSize: DWORD,
        rFieldOffsets: *const COR_FIELD_OFFSET,
        ulClassSize: ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().SetClassLayout)(self, td, dwPackSize, rFieldOffsets, ulClassSize)
    }
    pub unsafe fn DeleteClassLayout(&self, td: mdTypeDef) -> HRESULT {
        (self.i_metadata_emit().DeleteClassLayout)(self, td)
    }
    pub unsafe fn DeleteFieldMarshal(&self, tk: mdToken) -> HRESULT {
        (self.i_metadata_emit().DeleteFieldMarshal)(self, tk)
    }
    pub unsafe fn DefinePermissionSet(
        &self,
        tk: mdToken,
        dwAction: DWORD,
        pvPermission: *const c_void,
        cbPermission: ULONG,
        ppm: *mut mdPermission,
    ) -> HRESULT {
        (self.i_metadata_emit().DefinePermissionSet)(
            self,
            tk,
            dwAction,
            pvPermission,
            cbPermission,
            ppm,
        )
    }
    pub unsafe fn SetRVA(&self, md: mdMethodDef, ulRVA: ULONG) -> HRESULT {
        (self.i_metadata_emit().SetRVA)(self, md, ulRVA)
    }
    pub unsafe fn GetTokenFromSig(
        &self,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        pmsig: *mut mdSignature,
    ) -> HRESULT {
        (self.i_metadata_emit().GetTokenFromSig)(self, pvSig, cbSig, pmsig)
    }
    pub unsafe fn DefineModuleRef(&self, szName: LPCWSTR, pmur: *mut mdModuleRef) -> HRESULT {
        (self.i_metadata_emit().DefineModuleRef)(self, szName, pmur)
    }
    pub unsafe fn SetParent(&self, mr: mdMemberRef, tk: mdToken) -> HRESULT {
        (self.i_metadata_emit().SetParent)(self, mr, tk)
    }
    pub unsafe fn GetTokenFromTypeSpec(
        &self,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        ptypespec: *mut mdTypeSpec,
    ) -> HRESULT {
        (self.i_metadata_emit().GetTokenFromTypeSpec)(self, pvSig, cbSig, ptypespec)
    }
    pub unsafe fn SaveToMemory(&self, pbData: *mut c_void, cbData: ULONG) -> HRESULT {
        (self.i_metadata_emit().SaveToMemory)(self, pbData, cbData)
    }
    pub unsafe fn DefineUserString(
        &self,
        szString: LPCWSTR,
        cchString: ULONG,
        pstk: *mut mdString,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineUserString)(self, szString, cchString, pstk)
    }
    pub unsafe fn DeleteToken(&self, tkObj: mdToken) -> HRESULT {
        (self.i_metadata_emit().DeleteToken)(self, tkObj)
    }
    pub unsafe fn SetTypeDefProps(
        &self,
        td: mdTypeDef,
        dwTypeDefFlags: DWORD,
        tkExtends: mdToken,
        rtkImplements: *const mdToken,
    ) -> HRESULT {
        (self.i_metadata_emit().SetTypeDefProps)(self, td, dwTypeDefFlags, tkExtends, rtkImplements)
    }
    pub unsafe fn SetEventProps(
        &self,
        ev: mdEvent,
        dwEventFlags: DWORD,
        tkEventType: mdToken,
        mdAddOn: mdMethodDef,
        mdRemoveOn: mdMethodDef,
        mdFire: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
    ) -> HRESULT {
        (self.i_metadata_emit().SetEventProps)(
            self,
            ev,
            dwEventFlags,
            tkEventType,
            mdAddOn,
            mdRemoveOn,
            mdFire,
            rmdOtherMethods,
        )
    }
    pub unsafe fn SetPermissionSetProps(
        &self,
        tk: mdToken,
        dwAction: DWORD,
        pvPermission: *const c_void,
        cbPermission: ULONG,
        ppm: *mut mdPermission,
    ) -> HRESULT {
        (self.i_metadata_emit().SetPermissionSetProps)(
            self,
            tk,
            dwAction,
            pvPermission,
            cbPermission,
            ppm,
        )
    }
    pub unsafe fn DefinePinvokeMap(
        &self,
        tk: mdToken,
        dwMappingFlags: DWORD,
        szImportName: LPCWSTR,
        mrImportDLL: mdModuleRef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefinePinvokeMap)(
            self,
            tk,
            dwMappingFlags,
            szImportName,
            mrImportDLL,
        )
    }
    pub unsafe fn SetPinvokeMap(
        &self,
        tk: mdToken,
        dwMappingFlags: DWORD,
        szImportName: LPCWSTR,
        mrImportDLL: mdModuleRef,
    ) -> HRESULT {
        (self.i_metadata_emit().SetPinvokeMap)(self, tk, dwMappingFlags, szImportName, mrImportDLL)
    }
    pub unsafe fn DeletePinvokeMap(&self, tk: mdToken) -> HRESULT {
        (self.i_metadata_emit().DeletePinvokeMap)(self, tk)
    }
    pub unsafe fn DefineCustomAttribute(
        &self,
        tkOwner: mdToken,
        tkCtor: mdToken,
        pCustomAttribute: *const c_void,
        cbCustomAttribute: ULONG,
        pcv: *mut mdCustomAttribute,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineCustomAttribute)(
            self,
            tkOwner,
            tkCtor,
            pCustomAttribute,
            cbCustomAttribute,
            pcv,
        )
    }
    pub unsafe fn SetCustomAttributeValue(
        &self,
        pcv: mdCustomAttribute,
        pCustomAttribute: *const c_void,
        cbCustomAttribute: ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().SetCustomAttributeValue)(
            self,
            pcv,
            pCustomAttribute,
            cbCustomAttribute,
        )
    }
    pub unsafe fn DefineField(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        dwFieldFlags: DWORD,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        pmd: *mut mdFieldDef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineField)(
            self,
            td,
            szName,
            dwFieldFlags,
            pvSigBlob,
            cbSigBlob,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
            pmd,
        )
    }
    pub unsafe fn DefineProperty(
        &self,
        td: mdTypeDef,
        szProperty: LPCWSTR,
        dwPropFlags: DWORD,
        pvSig: PCCOR_SIGNATURE,
        cbSig: ULONG,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        mdSetter: mdMethodDef,
        mdGetter: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
        pmdProp: *mut mdProperty,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineProperty)(
            self,
            td,
            szProperty,
            dwPropFlags,
            pvSig,
            cbSig,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
            mdSetter,
            mdGetter,
            rmdOtherMethods,
            pmdProp,
        )
    }
    pub unsafe fn DefineParam(
        &self,
        md: mdMethodDef,
        ulParamSeq: ULONG,
        szName: LPCWSTR,
        dwParamFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        ppd: *mut mdParamDef,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineParam)(
            self,
            md,
            ulParamSeq,
            szName,
            dwParamFlags,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
            ppd,
        )
    }
    pub unsafe fn SetFieldProps(
        &self,
        fd: mdFieldDef,
        dwFieldFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().SetFieldProps)(
            self,
            fd,
            dwFieldFlags,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
        )
    }
    pub unsafe fn SetPropertyProps(
        &self,
        pr: mdProperty,
        dwPropFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *const c_void,
        cchValue: ULONG,
        mdSetter: mdMethodDef,
        mdGetter: mdMethodDef,
        rmdOtherMethods: *const mdMethodDef,
    ) -> HRESULT {
        (self.i_metadata_emit().SetPropertyProps)(
            self,
            pr,
            dwPropFlags,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
            mdSetter,
            mdGetter,
            rmdOtherMethods,
        )
    }
    pub unsafe fn SetParamProps(
        &self,
        pd: mdParamDef,
        szName: LPCWSTR,
        dwParamFlags: DWORD,
        dwCPlusTypeFlag: DWORD,
        pValue: *mut c_void,
        cchValue: ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().SetParamProps)(
            self,
            pd,
            szName,
            dwParamFlags,
            dwCPlusTypeFlag,
            pValue,
            cchValue,
        )
    }
    pub unsafe fn DefineSecurityAttributeSet(
        &self,
        tkObj: mdToken,
        rSecAttrs: *const COR_SECATTR,
        cSecAttrs: ULONG,
        pulErrorAttr: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().DefineSecurityAttributeSet)(
            self,
            tkObj,
            rSecAttrs,
            cSecAttrs,
            pulErrorAttr,
        )
    }
    pub unsafe fn ApplyEditAndContinue(
        &self,
        pImport: *const Unknown, // TODO: Which actual class?
    ) -> HRESULT {
        (self.i_metadata_emit().ApplyEditAndContinue)(self, pImport)
    }
    pub unsafe fn TranslateSigWithScope(
        &self,
        pAssemImport: *const MetaDataAssemblyImport,
        pbHashValue: *const c_void,
        cbHashValue: ULONG,
        import: *const MetaDataImport,
        pbSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pAssemEmit: *const MetaDataAssemblyEmit,
        emit: *const Self,
        pvTranslatedSig: PCOR_SIGNATURE,
        cbTranslatedSigMax: ULONG,
        pcbTranslatedSig: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_emit().TranslateSigWithScope)(
            self,
            pAssemImport,
            pbHashValue,
            cbHashValue,
            import,
            pbSigBlob,
            cbSigBlob,
            pAssemEmit,
            emit,
            pvTranslatedSig,
            cbTranslatedSigMax,
            pcbTranslatedSig,
        )
    }
    pub unsafe fn SetMethodImplFlags(&self, md: mdMethodDef, dwImplFlags: DWORD) -> HRESULT {
        (self.i_metadata_emit().SetMethodImplFlags)(self, md, dwImplFlags)
    }
    pub unsafe fn SetFieldRVA(&self, fd: mdFieldDef, ulRVA: ULONG) -> HRESULT {
        (self.i_metadata_emit().SetFieldRVA)(self, fd, ulRVA)
    }
    pub unsafe fn Merge(
        &self,
        pImport: *const MetaDataImport,
        pHostMapToken: *const Unknown, // TODO: Implement IMapToken and MapToken coclass
        pHandler: *const Unknown,
    ) -> HRESULT {
        (self.i_metadata_emit().Merge)(self, pImport, pHostMapToken, pHandler)
    }
    pub unsafe fn MergeEnd(&self) -> HRESULT {
        (self.i_metadata_emit().MergeEnd)(self)
    }
    pub unsafe fn DefineMethodSpecfn(
        &self,
        tkParent: mdToken,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmi: *mut mdMethodSpec,
    ) -> HRESULT {
        (self.i_metadata_emit_2().DefineMethodSpec)(self, tkParent, pvSigBlob, cbSigBlob, pmi)
    }
    pub unsafe fn GetDeltaSaveSize(&self, fSave: CorSaveSize, pdwSaveSize: *mut DWORD) -> HRESULT {
        (self.i_metadata_emit_2().GetDeltaSaveSize)(self, fSave, pdwSaveSize)
    }
    pub unsafe fn SaveDelta(&self, szFile: LPCWSTR, dwSaveFlags: DWORD) -> HRESULT {
        (self.i_metadata_emit_2().SaveDelta)(self, szFile, dwSaveFlags)
    }
    pub unsafe fn SaveDeltaToStream(
        &self,
        pIStream: *const Unknown, // TODO: IStream
        dwSaveFlags: DWORD,
    ) -> HRESULT {
        (self.i_metadata_emit_2().SaveDeltaToStream)(self, pIStream, dwSaveFlags)
    }
    pub unsafe fn SaveDeltaToMemory(&self, pbData: *mut c_void, cbData: ULONG) -> HRESULT {
        (self.i_metadata_emit_2().SaveDeltaToMemory)(self, pbData, cbData)
    }
    pub unsafe fn DefineGenericParam(
        &self,
        tk: mdToken,
        ulParamSeq: ULONG,
        dwParamFlags: DWORD,
        szname: LPCWSTR,
        reserved: DWORD,
        rtkConstraints: *const mdToken,
        pgp: *mut mdGenericParam,
    ) -> HRESULT {
        (self.i_metadata_emit_2().DefineGenericParam)(
            self,
            tk,
            ulParamSeq,
            dwParamFlags,
            szname,
            reserved,
            rtkConstraints,
            pgp,
        )
    }
    pub unsafe fn SetGenericParamProps(
        &self,
        gp: mdGenericParam,
        dwParamFlags: DWORD,
        szName: LPCWSTR,
        reserved: DWORD,
        rtkConstraints: *const mdToken,
    ) -> HRESULT {
        (self.i_metadata_emit_2().SetGenericParamProps)(
            self,
            gp,
            dwParamFlags,
            szName,
            reserved,
            rtkConstraints,
        )
    }
    pub unsafe fn ResetENCLog(&self) -> HRESULT {
        (self.i_metadata_emit_2().ResetENCLog)(self)
    }
}
