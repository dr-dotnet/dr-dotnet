#![allow(non_snake_case)]
use crate::ffi::{
    int, mdCustomAttribute, mdEvent, mdFieldDef, mdGenericParam, mdGenericParamConstraint,
    mdInterfaceImpl, mdMemberRef, mdMethodDef, mdMethodSpec, mdModule, mdModuleRef, mdParamDef,
    mdPermission, mdProperty, mdSignature, mdString, mdToken, mdTypeDef, mdTypeRef, mdTypeSpec,
    IMetaDataImport, IMetaDataImport2, IUnknown, BOOL, COR_FIELD_OFFSET, DWORD, GUID, HCORENUM,
    HRESULT, LPCWSTR, MDUTF8CSTR, PCCOR_SIGNATURE, REFIID, ULONG, UVCP_CONSTANT, WCHAR,
};
use std::ffi::c_void;

#[repr(C)]
pub struct MetaDataImportVtbl {
    pub IUnknown: IUnknown<MetaDataImport>,
    pub IMetaDataImport: IMetaDataImport<MetaDataImport>,
    pub IMetaDataImport2: IMetaDataImport2<MetaDataImport>,
}

#[repr(C)]
pub struct MetaDataImport {
    pub lpVtbl: *const MetaDataImportVtbl,
}

impl MetaDataImport {
    pub unsafe fn i_metadata_import(&self) -> &IMetaDataImport<Self> {
        &(*self.lpVtbl).IMetaDataImport
    }
    pub unsafe fn i_metadata_import_2(&self) -> &IMetaDataImport2<Self> {
        &(*self.lpVtbl).IMetaDataImport2
    }
    pub unsafe fn CloseEnum(&self, hEnum: HCORENUM) -> () {
        (self.i_metadata_import().CloseEnum)(self, hEnum)
    }
    pub unsafe fn CountEnum(&self, hEnum: HCORENUM, pulCount: *mut ULONG) -> HRESULT {
        (self.i_metadata_import().CountEnum)(self, hEnum, pulCount)
    }
    pub unsafe fn ResetEnum(&self, hEnum: HCORENUM, ulPos: *const ULONG) -> HRESULT {
        (self.i_metadata_import().ResetEnum)(self, hEnum, ulPos)
    }
    pub unsafe fn EnumTypeDefs(
        &self,
        phEnum: *mut HCORENUM,
        rTypeDefs: *const mdTypeDef,
        cMax: ULONG,
        pcTypeDefs: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumTypeDefs)(self, phEnum, rTypeDefs, cMax, pcTypeDefs)
    }
    pub unsafe fn EnumInterfaceImpls(
        &self,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rImpls: *mut mdInterfaceImpl,
        cMax: ULONG,
        pcImpls: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumInterfaceImpls)(self, phEnum, td, rImpls, cMax, pcImpls)
    }
    pub unsafe fn EnumTypeRefs(
        &self,
        phEnum: *mut HCORENUM,
        rTypeRefs: *mut mdTypeRef,
        cMax: ULONG,
        pcTypeRefs: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumTypeRefs)(self, phEnum, rTypeRefs, cMax, pcTypeRefs)
    }
    pub unsafe fn FindTypeDefByName(
        &self,
        szTypeDef: LPCWSTR,
        tkEnclosingClass: mdToken,
        ptd: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_metadata_import().FindTypeDefByName)(self, szTypeDef, tkEnclosingClass, ptd)
    }
    pub unsafe fn GetScopeProps(
        &self,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pmvid: *mut GUID,
    ) -> HRESULT {
        (self.i_metadata_import().GetScopeProps)(self, szName, cchName, pchName, pmvid)
    }
    pub unsafe fn GetModuleFromScope(&self, pmd: *mut mdModule) -> HRESULT {
        (self.i_metadata_import().GetModuleFromScope)(self, pmd)
    }
    pub unsafe fn GetTypeDefProps(
        &self,
        td: mdTypeDef,
        szTypeDef: *mut WCHAR,
        cchTypeDef: ULONG,
        pchTypeDef: *mut ULONG,
        pdwTypeDefFlags: *mut DWORD,
        ptkExtends: *mut mdToken,
    ) -> HRESULT {
        (self.i_metadata_import().GetTypeDefProps)(
            self,
            td,
            szTypeDef,
            cchTypeDef,
            pchTypeDef,
            pdwTypeDefFlags,
            ptkExtends,
        )
    }
    pub unsafe fn GetInterfaceImplProps(
        &self,
        iiImpl: mdInterfaceImpl,
        pClass: *mut mdTypeDef,
        ptkIface: *mut mdToken,
    ) -> HRESULT {
        (self.i_metadata_import().GetInterfaceImplProps)(self, iiImpl, pClass, ptkIface)
    }
    pub unsafe fn GetTypeRefProps(
        &self,
        tr: mdTypeRef,
        ptkResolutionScope: *mut mdToken,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetTypeRefProps)(
            self,
            tr,
            ptkResolutionScope,
            szName,
            cchName,
            pchName,
        )
    }
    pub unsafe fn ResolveTypeRef(
        &self,
        tr: mdTypeRef,
        riid: REFIID,
        ppIScope: *mut *mut Self, // TODO: What actual class here?
        ptd: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_metadata_import().ResolveTypeRef)(self, tr, riid, ppIScope, ptd)
    }
    pub unsafe fn EnumMembers(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rMembers: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMembers)(self, phEnum, cl, rMembers, cMax, pcTokens)
    }
    pub unsafe fn EnumMembersWithName(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rMembers: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMembersWithName)(
            self, phEnum, cl, szName, rMembers, cMax, pcTokens,
        )
    }
    pub unsafe fn EnumMethods(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rMethods: *mut mdMethodDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMethods)(self, phEnum, cl, rMethods, cMax, pcTokens)
    }
    pub unsafe fn EnumMethodsWithName(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rMethods: *mut mdMethodDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMethodsWithName)(
            self, phEnum, cl, szName, rMethods, cMax, pcTokens,
        )
    }
    pub unsafe fn EnumFields(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        rFields: *mut mdFieldDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumFields)(self, phEnum, cl, rFields, cMax, pcTokens)
    }
    pub unsafe fn EnumFieldsWithName(
        &self,
        phEnum: *mut HCORENUM,
        cl: mdTypeDef,
        szName: LPCWSTR,
        rFields: *mut mdFieldDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumFieldsWithName)(
            self, phEnum, cl, szName, rFields, cMax, pcTokens,
        )
    }
    pub unsafe fn EnumParams(
        &self,
        phEnum: *mut HCORENUM,
        mb: mdMethodDef,
        rParams: *mut mdParamDef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumParams)(self, phEnum, mb, rParams, cMax, pcTokens)
    }
    pub unsafe fn EnumMemberRefs(
        &self,
        phEnum: *mut HCORENUM,
        tkParent: mdToken,
        rMemberRefs: *mut mdMemberRef,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMemberRefs)(
            self,
            phEnum,
            tkParent,
            rMemberRefs,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn EnumMethodImpls(
        &self,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rMethodBody: *mut mdToken,
        rMethodDecl: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMethodImpls)(
            self,
            phEnum,
            td,
            rMethodBody,
            rMethodDecl,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn EnumPermissionSets(
        &self,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        dwActions: DWORD,
        rPermission: *mut mdPermission,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumPermissionSets)(
            self,
            phEnum,
            tk,
            dwActions,
            rPermission,
            cMax,
            pcTokens,
        )
    }
    pub unsafe fn FindMember(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdToken,
    ) -> HRESULT {
        (self.i_metadata_import().FindMember)(self, td, szName, pvSigBlob, cbSigBlob, pmb)
    }
    pub unsafe fn FindMethod(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdMethodDef,
    ) -> HRESULT {
        (self.i_metadata_import().FindMethod)(self, td, szName, pvSigBlob, cbSigBlob, pmb)
    }
    pub unsafe fn FindField(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdFieldDef,
    ) -> HRESULT {
        (self.i_metadata_import().FindField)(self, td, szName, pvSigBlob, cbSigBlob, pmb)
    }
    pub unsafe fn FindMemberRef(
        &self,
        td: mdTypeDef,
        szName: LPCWSTR,
        pvSigBlob: PCCOR_SIGNATURE,
        cbSigBlob: ULONG,
        pmb: *mut mdMemberRef,
    ) -> HRESULT {
        (self.i_metadata_import().FindMemberRef)(self, td, szName, pvSigBlob, cbSigBlob, pmb)
    }
    pub unsafe fn GetMethodProps(
        &self,
        mb: mdMethodDef,
        pClass: *mut mdTypeDef,
        szMethod: *mut WCHAR,
        cchMethod: ULONG,
        pchMethod: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_import().GetMethodProps)(
            self,
            mb,
            pClass,
            szMethod,
            cchMethod,
            pchMethod,
            pdwAttr,
            ppvSigBlob,
            pcbSigBlob,
            pulCodeRVA,
            pdwImplFlags,
        )
    }
    pub unsafe fn GetMemberRefProps(
        &self,
        mr: mdMemberRef,
        ptk: *mut mdToken,
        szMember: *mut WCHAR,
        cchMember: ULONG,
        pchMember: *mut ULONG,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetMemberRefProps)(
            self, mr, ptk, szMember, cchMember, pchMember, ppvSigBlob, pbSig,
        )
    }
    pub unsafe fn EnumProperties(
        &self,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rProperties: *mut mdProperty,
        cMax: ULONG,
        pcProperties: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumProperties)(self, phEnum, td, rProperties, cMax, pcProperties)
    }
    pub unsafe fn EnumEvents(
        &self,
        phEnum: *mut HCORENUM,
        td: mdTypeDef,
        rEvents: *mut mdEvent,
        cMax: ULONG,
        pcEvents: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumEvents)(self, phEnum, td, rEvents, cMax, pcEvents)
    }
    pub unsafe fn GetEventProps(
        &self,
        ev: mdEvent,
        pClass: *mut mdTypeDef,
        szEvent: *mut WCHAR,
        cchEvent: ULONG,
        pchEvent: *mut ULONG,
        pdwEventFlags: *mut DWORD,
        ptkEventType: *mut mdToken,
        pmdAddOn: *mut mdMethodDef,
        pmdRemoveOn: *mut mdMethodDef,
        pmdFire: *mut mdMethodDef,
        rmdOtherMethod: *mut mdMethodDef,
        cMax: ULONG,
        pcOtherMethod: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetEventProps)(
            self,
            ev,
            pClass,
            szEvent,
            cchEvent,
            pchEvent,
            pdwEventFlags,
            ptkEventType,
            pmdAddOn,
            pmdRemoveOn,
            pmdFire,
            rmdOtherMethod,
            cMax,
            pcOtherMethod,
        )
    }
    pub unsafe fn EnumMethodSemantics(
        &self,
        phEnum: *mut HCORENUM,
        mb: mdMethodDef,
        rEventProp: *mut mdToken,
        cMax: ULONG,
        pcEventProp: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumMethodSemantics)(
            self,
            phEnum,
            mb,
            rEventProp,
            cMax,
            pcEventProp,
        )
    }
    pub unsafe fn GetMethodSemantics(
        &self,
        mb: mdMethodDef,
        tkEventProp: mdToken,
        pdwSemanticsFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_import().GetMethodSemantics)(self, mb, tkEventProp, pdwSemanticsFlags)
    }
    pub unsafe fn GetClassLayout(
        &self,
        td: mdTypeDef,
        pdwPackSize: *mut DWORD,
        rFieldOffset: *mut COR_FIELD_OFFSET,
        cMax: ULONG,
        pcFieldOffset: *mut ULONG,
        pulClassSize: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetClassLayout)(
            self,
            td,
            pdwPackSize,
            rFieldOffset,
            cMax,
            pcFieldOffset,
            pulClassSize,
        )
    }
    pub unsafe fn GetFieldMarshal(
        &self,
        tk: mdToken,
        ppvNativeType: *mut PCCOR_SIGNATURE,
        pcbNativeType: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetFieldMarshal)(self, tk, ppvNativeType, pcbNativeType)
    }
    pub unsafe fn GetRVA(
        &self,
        tk: mdToken,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_import().GetRVA)(self, tk, pulCodeRVA, pdwImplFlags)
    }
    pub unsafe fn GetPermissionSetProps(
        &self,
        pm: mdPermission,
        pdwAction: *mut DWORD,
        ppvPermission: *mut *mut c_void,
        pcbPermission: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetPermissionSetProps)(
            self,
            pm,
            pdwAction,
            ppvPermission,
            pcbPermission,
        )
    }
    pub unsafe fn GetSigFromToken(
        &self,
        mdSig: mdSignature,
        ppvSig: *mut PCCOR_SIGNATURE,
        pcbSig: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetSigFromToken)(self, mdSig, ppvSig, pcbSig)
    }
    pub unsafe fn GetModuleRefProps(
        &self,
        mur: mdModuleRef,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetModuleRefProps)(self, mur, szName, cchName, pchName)
    }
    pub unsafe fn EnumModuleRefs(
        &self,
        phEnum: *mut HCORENUM,
        rModuleRefs: *mut mdModuleRef,
        cmax: ULONG,
        pcModuleRefs: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumModuleRefs)(self, phEnum, rModuleRefs, cmax, pcModuleRefs)
    }
    pub unsafe fn GetTypeSpecFromToken(
        &self,
        typespec: mdTypeSpec,
        ppvSig: *mut PCCOR_SIGNATURE,
        pcbSig: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetTypeSpecFromToken)(self, typespec, ppvSig, pcbSig)
    }
    pub unsafe fn GetNameFromToken(&self, tk: mdToken, pszUtf8NamePtr: *mut MDUTF8CSTR) -> HRESULT {
        (self.i_metadata_import().GetNameFromToken)(self, tk, pszUtf8NamePtr)
    }
    pub unsafe fn EnumUnresolvedMethods(
        &self,
        phEnum: *mut HCORENUM,
        rMethods: *mut mdToken,
        cMax: ULONG,
        pcTokens: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumUnresolvedMethods)(self, phEnum, rMethods, cMax, pcTokens)
    }
    pub unsafe fn GetUserString(
        &self,
        stk: mdString,
        szString: *mut WCHAR,
        cchString: ULONG,
        pchString: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetUserString)(self, stk, szString, cchString, pchString)
    }
    pub unsafe fn GetPinvokeMap(
        &self,
        tk: mdToken,
        pdwMappingFlags: *mut DWORD,
        szImportName: *mut WCHAR,
        cchImportName: ULONG,
        pchImportName: *mut ULONG,
        pmrImportDLL: *mut mdModuleRef,
    ) -> HRESULT {
        (self.i_metadata_import().GetPinvokeMap)(
            self,
            tk,
            pdwMappingFlags,
            szImportName,
            cchImportName,
            pchImportName,
            pmrImportDLL,
        )
    }
    pub unsafe fn EnumSignatures(
        &self,
        phEnum: *mut HCORENUM,
        rSignatures: *mut mdSignature,
        cMax: ULONG,
        pcSignatures: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumSignatures)(self, phEnum, rSignatures, cMax, pcSignatures)
    }
    pub unsafe fn EnumTypeSpecs(
        &self,
        phEnum: *mut HCORENUM,
        rTypeSpecs: *mut mdTypeSpec,
        cMax: ULONG,
        pcTypeSpecs: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumTypeSpecs)(self, phEnum, rTypeSpecs, cMax, pcTypeSpecs)
    }
    pub unsafe fn EnumUserStrings(
        &self,
        phEnum: *mut HCORENUM,
        rStrings: *mut mdString,
        cMax: ULONG,
        pcStrings: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumUserStrings)(self, phEnum, rStrings, cMax, pcStrings)
    }
    pub unsafe fn GetParamForMethodIndex(
        &self,
        md: mdMethodDef,
        ulParamSeq: ULONG,
        ppd: *mut mdParamDef,
    ) -> HRESULT {
        (self.i_metadata_import().GetParamForMethodIndex)(self, md, ulParamSeq, ppd)
    }
    pub unsafe fn EnumCustomAttributes(
        &self,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        tkType: mdToken,
        rCustomAttributes: *mut mdCustomAttribute,
        cMax: ULONG,
        pcCustomAttributes: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().EnumCustomAttributes)(
            self,
            phEnum,
            tk,
            tkType,
            rCustomAttributes,
            cMax,
            pcCustomAttributes,
        )
    }
    pub unsafe fn GetCustomAttributeProps(
        &self,
        cv: mdCustomAttribute,
        ptkObj: *mut mdToken,
        ptkType: *mut mdToken,
        ppBlob: *mut *mut c_void,
        pcbSize: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetCustomAttributeProps)(
            self, cv, ptkObj, ptkType, ppBlob, pcbSize,
        )
    }
    pub unsafe fn FindTypeRef(
        &self,
        tkResolutionScope: mdToken,
        szName: LPCWSTR,
        ptr: *mut mdTypeRef,
    ) -> HRESULT {
        (self.i_metadata_import().FindTypeRef)(self, tkResolutionScope, szName, ptr)
    }
    pub unsafe fn GetMemberProps(
        &self,
        mb: mdToken,
        pClass: *mut mdTypeDef,
        szMember: *mut WCHAR,
        cchMember: ULONG,
        pchMember: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pulCodeRVA: *mut ULONG,
        pdwImplFlags: *mut DWORD,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetMemberProps)(
            self,
            mb,
            pClass,
            szMember,
            cchMember,
            pchMember,
            pdwAttr,
            ppvSigBlob,
            pcbSigBlob,
            pulCodeRVA,
            pdwImplFlags,
            pdwCPlusTypeFlag,
            ppValue,
            pcchValue,
        )
    }
    pub unsafe fn GetFieldProps(
        &self,
        mb: mdToken,
        pClass: *mut mdTypeDef,
        szField: *mut WCHAR,
        cchField: ULONG,
        pchField: *mut ULONG,
        pdwAttr: *mut DWORD,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetFieldProps)(
            self,
            mb,
            pClass,
            szField,
            cchField,
            pchField,
            pdwAttr,
            ppvSigBlob,
            pcbSigBlob,
            pdwCPlusTypeFlag,
            ppValue,
            pcchValue,
        )
    }
    pub unsafe fn GetPropertyProps(
        &self,
        prop: mdProperty,
        pClass: *mut mdTypeDef,
        szProperty: *mut WCHAR,
        cchProperty: ULONG,
        pchProperty: *mut ULONG,
        pdwPropFlags: *mut DWORD,
        ppvSig: *mut PCCOR_SIGNATURE,
        pbSig: *mut ULONG,
        pdwCPlusTypeFlag: *mut DWORD,
        ppDefaultValue: *mut UVCP_CONSTANT,
        pcchDefaultValue: *mut ULONG,
        pmdSetter: *mut mdMethodDef,
        pmdGetter: *mut mdMethodDef,
        rmdOtherMethod: *mut mdMethodDef,
        cMax: ULONG,
        pcOtherMethod: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetPropertyProps)(
            self,
            prop,
            pClass,
            szProperty,
            cchProperty,
            pchProperty,
            pdwPropFlags,
            ppvSig,
            pbSig,
            pdwCPlusTypeFlag,
            ppDefaultValue,
            pcchDefaultValue,
            pmdSetter,
            pmdGetter,
            rmdOtherMethod,
            cMax,
            pcOtherMethod,
        )
    }
    pub unsafe fn GetParamProps(
        &self,
        tk: mdParamDef,
        pmd: *mut mdMethodDef,
        pulSequence: *mut ULONG,
        szName: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
        pdwAttr: *mut DWORD,
        pdwCPlusTypeFlag: *mut DWORD,
        ppValue: *mut UVCP_CONSTANT,
        pcchValue: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetParamProps)(
            self,
            tk,
            pmd,
            pulSequence,
            szName,
            cchName,
            pchName,
            pdwAttr,
            pdwCPlusTypeFlag,
            ppValue,
            pcchValue,
        )
    }
    pub unsafe fn GetCustomAttributeByName(
        &self,
        tkObj: mdToken,
        szName: LPCWSTR,
        ppData: *mut *mut c_void,
        pcbData: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetCustomAttributeByName)(self, tkObj, szName, ppData, pcbData)
    }
    pub unsafe fn IsValidToken(&self, tk: mdToken) -> BOOL {
        (self.i_metadata_import().IsValidToken)(self, tk)
    }
    pub unsafe fn GetNestedClassProps(
        &self,
        tdNestedClass: mdTypeDef,
        ptdEnclosingClass: *mut mdTypeDef,
    ) -> HRESULT {
        (self.i_metadata_import().GetNestedClassProps)(self, tdNestedClass, ptdEnclosingClass)
    }
    pub unsafe fn GetNativeCallConvFromSig(
        &self,
        pvSig: *const c_void,
        cbSig: ULONG,
        pCallConv: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import().GetNativeCallConvFromSig)(self, pvSig, cbSig, pCallConv)
    }
    pub unsafe fn IsGlobal(&self, pd: mdToken, pbGlobal: *mut int) -> HRESULT {
        (self.i_metadata_import().IsGlobal)(self, pd, pbGlobal)
    }
    pub unsafe fn EnumGenericParams(
        &self,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        rGenericParams: *mut mdGenericParam,
        cMax: ULONG,
        pcGenericParams: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import_2().EnumGenericParams)(
            self,
            phEnum,
            tk,
            rGenericParams,
            cMax,
            pcGenericParams,
        )
    }
    pub unsafe fn GetGenericParamProps(
        &self,
        gp: mdGenericParam,
        pulParamSeq: *mut ULONG,
        pdwParamFlags: *mut DWORD,
        ptOwner: *mut mdToken,
        reserved: *mut DWORD,
        wzname: *mut WCHAR,
        cchName: ULONG,
        pchName: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import_2().GetGenericParamProps)(
            self,
            gp,
            pulParamSeq,
            pdwParamFlags,
            ptOwner,
            reserved,
            wzname,
            cchName,
            pchName,
        )
    }
    pub unsafe fn GetMethodSpecProps(
        &self,
        mi: mdMethodSpec,
        tkParent: *mut mdToken,
        ppvSigBlob: *mut PCCOR_SIGNATURE,
        pcbSigBlob: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import_2().GetMethodSpecProps)(self, mi, tkParent, ppvSigBlob, pcbSigBlob)
    }
    pub unsafe fn EnumGenericParamConstraints(
        &self,
        phEnum: *mut HCORENUM,
        tk: mdGenericParam,
        rGenericParamConstraints: *mut mdGenericParamConstraint,
        cMax: ULONG,
        pcGenericParamConstraints: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import_2().EnumGenericParamConstraints)(
            self,
            phEnum,
            tk,
            rGenericParamConstraints,
            cMax,
            pcGenericParamConstraints,
        )
    }
    pub unsafe fn GetGenericParamConstraintProps(
        &self,
        gpc: mdGenericParamConstraint,
        ptGenericParam: *mut mdGenericParam,
        ptkConstraintType: *mut mdToken,
    ) -> HRESULT {
        (self.i_metadata_import_2().GetGenericParamConstraintProps)(
            self,
            gpc,
            ptGenericParam,
            ptkConstraintType,
        )
    }
    pub unsafe fn GetPEKind(&self, pdwPEKind: *mut DWORD, pdwMAchine: *mut DWORD) -> HRESULT {
        (self.i_metadata_import_2().GetPEKind)(self, pdwPEKind, pdwMAchine)
    }
    pub unsafe fn GetVersionString(
        &self,
        pwzBuf: *mut WCHAR,
        ccBufSize: DWORD,
        pccBufSize: *mut DWORD,
    ) -> HRESULT {
        (self.i_metadata_import_2().GetVersionString)(self, pwzBuf, ccBufSize, pccBufSize)
    }
    pub unsafe fn EnumMethodSpecs(
        &self,
        phEnum: *mut HCORENUM,
        tk: mdToken,
        rMethodSpecs: *mut mdMethodSpec,
        cMax: ULONG,
        pcMethodSpecs: *mut ULONG,
    ) -> HRESULT {
        (self.i_metadata_import_2().EnumMethodSpecs)(
            self,
            phEnum,
            tk,
            rMethodSpecs,
            cMax,
            pcMethodSpecs,
        )
    }
}
