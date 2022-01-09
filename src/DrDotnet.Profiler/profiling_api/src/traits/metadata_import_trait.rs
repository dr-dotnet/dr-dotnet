use crate::{
    ffi::{mdMethodDef, mdTypeDef, HRESULT},
    MethodProps, TypeProps
};

pub trait MetadataImportTrait {
    fn get_method_props(&self, mb: mdMethodDef) -> Result<MethodProps, HRESULT>;
    fn get_type_def_props(&self, td: mdTypeDef) -> Result<TypeProps, HRESULT>;
}