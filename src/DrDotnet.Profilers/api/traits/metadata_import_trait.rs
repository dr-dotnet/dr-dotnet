use crate::{
    ffi::{mdMethodDef, mdTypeDef, HRESULT},
    MethodProps, TypeProps,
};

pub trait MetadataImportTrait {
    fn get_method_props(&self, mb: mdMethodDef) -> Result<MethodProps, HRESULT>;
    fn get_type_def_props(&self, td: mdTypeDef) -> Result<TypeProps, HRESULT>;
    fn get_nested_class_props(&self, td: crate::ffi::mdTypeDef) -> Result<crate::ffi::mdTypeDef, HRESULT>;
    fn enum_generic_params(&self, td: crate::ffi::mdTypeDef) -> Result<Vec<crate::ffi::mdGenericParam>, HRESULT>;
    // We could return more than just the mdTypeDef, it just needs to be implemented
    fn get_generic_params_props(&self, td: crate::ffi::mdGenericParam) -> Result<crate::ffi::mdTypeDef, HRESULT>;
    fn get_version_string(&self) -> Result<String, HRESULT>;
}
