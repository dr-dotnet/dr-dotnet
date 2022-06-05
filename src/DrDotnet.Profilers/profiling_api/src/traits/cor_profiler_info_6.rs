use crate::{
    ffi::{mdMethodDef, ModuleID, HRESULT},
    CorProfilerInfo5, EnumNgenModuleMethodsInliningThisMethod,
};

pub trait CorProfilerInfo6: CorProfilerInfo5 {
    fn enum_ngen_module_methods_inlining_this_method(
        &self,
        inliners_module_id: ModuleID,
        inlinee_module_id: ModuleID,
        inlinee_method_id: mdMethodDef,
    ) -> Result<EnumNgenModuleMethodsInliningThisMethod, HRESULT>;
}
