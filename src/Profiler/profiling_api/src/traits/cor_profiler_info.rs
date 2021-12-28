use crate::{
    ffi::{
        mdMethodDef, AppDomainID, AssemblyID, ClassID, ContextID, CorOpenFlags, FunctionEnter,
        FunctionID, FunctionIDMapper, FunctionLeave, FunctionTailcall, MethodMalloc, ModuleID,
        ObjectID, ThreadID, COR_DEBUG_IL_TO_NATIVE_MAP, COR_IL_MAP, COR_PRF_MONITOR, DWORD, HANDLE,
        HRESULT, LPCBYTE,
    },
    AppDomainInfo, ArrayClassInfo, AssemblyInfo, ClassInfo, FunctionInfo, FunctionTokenAndMetadata,
    IlFunctionBody, MetadataImport, ModuleInfo,
};

pub trait CorProfilerInfo {
    fn get_class_from_object(&self, object_id: ObjectID) -> Result<ClassID, HRESULT>; // TODO: If class id result is null ptr, will HRESULT be an error?
    fn get_event_mask(&self) -> Result<COR_PRF_MONITOR, HRESULT>;
    fn get_function_from_ip(&self, ip: LPCBYTE) -> Result<FunctionID, HRESULT>;
    fn get_handle_from_thread(&self, thread_id: ThreadID) -> Result<HANDLE, HRESULT>;
    fn is_array_class(&self, class_id: ClassID) -> Result<ArrayClassInfo, HRESULT>;
    fn get_thread_info(&self, thread_id: ThreadID) -> Result<DWORD, HRESULT>;
    fn get_current_thread_id(&self) -> Result<ThreadID, HRESULT>;
    fn get_class_id_info(&self, class_id: ClassID) -> Result<ClassInfo, HRESULT>;
    fn get_function_info(&self, function_id: FunctionID) -> Result<FunctionInfo, HRESULT>;
    fn set_event_mask(&self, events: COR_PRF_MONITOR) -> Result<(), HRESULT>;
    fn set_enter_leave_function_hooks(
        &self,
        func_enter: FunctionEnter,
        func_leave: FunctionLeave,
        func_tailcall: FunctionTailcall,
    ) -> Result<(), HRESULT>;
    fn set_function_id_mapper(&self, func: FunctionIDMapper) -> Result<(), HRESULT>;
    fn get_token_and_metadata_from_function(
        &self,
        function_id: FunctionID,
    ) -> Result<FunctionTokenAndMetadata, HRESULT>;
    fn get_module_info(&self, module_id: ModuleID) -> Result<ModuleInfo, HRESULT>;
    fn get_module_metadata(
        &self,
        module_id: ModuleID,
        open_flags: CorOpenFlags,
    ) -> Result<MetadataImport, HRESULT>;
    fn get_il_function_body(
        &self,
        module_id: ModuleID,
        method_id: mdMethodDef,
    ) -> Result<IlFunctionBody, HRESULT>;
    fn get_il_function_body_allocator(
        &self,
        module_id: ModuleID,
    ) -> Result<&mut MethodMalloc, HRESULT>;
    fn set_il_function_body(
        &self,
        module_id: ModuleID,
        method_id: mdMethodDef,
        new_il_method_header: LPCBYTE,
    ) -> Result<(), HRESULT>;
    fn get_app_domain_info(&self, app_domain_id: AppDomainID) -> Result<AppDomainInfo, HRESULT>;
    fn get_assembly_info(&self, assembly_id: AssemblyID) -> Result<AssemblyInfo, HRESULT>;
    fn force_gc(&self) -> Result<(), HRESULT>;
    fn set_il_instrumented_code_map(
        &self,
        function_id: FunctionID,
        start_jit: bool,
        il_map_entries: &[COR_IL_MAP],
    ) -> Result<(), HRESULT>;
    fn get_thread_context(&self, thread_id: ThreadID) -> Result<ContextID, HRESULT>;
    fn get_il_to_native_mapping(
        &self,
        function_id: FunctionID,
    ) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT>;
}
