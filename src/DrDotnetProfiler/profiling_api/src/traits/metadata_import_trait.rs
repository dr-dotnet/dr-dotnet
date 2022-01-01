use crate::{
    ffi::{mdMethodDef, HRESULT},
    MethodProps,
};

pub trait MetadataImportTrait {
    fn get_method_props(&self, mb: mdMethodDef) -> Result<MethodProps, HRESULT>;
}
