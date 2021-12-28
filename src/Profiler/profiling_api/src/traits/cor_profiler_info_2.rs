use crate::{
    ffi::{
        mdFieldDef, mdMethodDef, mdTypeDef, AppDomainID, ClassID, ContextID, FunctionEnter2,
        FunctionID, FunctionLeave2, FunctionTailcall2, ModuleID, ObjectID, StackSnapshotCallback,
        ThreadID, BYTE, COR_PRF_CODE_INFO, COR_PRF_EX_CLAUSE_INFO, COR_PRF_FRAME_INFO,
        COR_PRF_GC_GENERATION_RANGE, COR_PRF_SNAPSHOT_INFO, COR_PRF_STATIC_TYPE, HRESULT,
    },
    ArrayObjectInfo, ClassInfo2, ClassLayout, CorProfilerInfo, FunctionInfo2,
};
use std::ffi::c_void;

pub trait CorProfilerInfo2: CorProfilerInfo {
    fn do_stack_snapshot(
        &self,
        thread: ThreadID,
        callback: StackSnapshotCallback,
        info_flags: COR_PRF_SNAPSHOT_INFO,
        client_data: *const c_void, // TODO: How will ownership of this client_data work? Needs to leak, what about cleanup?
        context: *const BYTE, // TODO: This should be a Win32 CONTEXT structure. This is CPU-arch dependent though, how to implement? What about ownership?
        context_size: u32,
    ) -> Result<(), HRESULT>;
    fn set_enter_leave_function_hooks_2(
        &self,
        func_enter: FunctionEnter2,
        func_leave: FunctionLeave2,
        func_tailcall: FunctionTailcall2,
    ) -> Result<(), HRESULT>;
    fn get_function_info_2(
        &self,
        func_id: FunctionID,
        frame_info: COR_PRF_FRAME_INFO,
    ) -> Result<FunctionInfo2, HRESULT>;
    fn get_class_layout(&self, class_id: ClassID) -> Result<ClassLayout, HRESULT>;
    fn get_class_id_info_2(&self, class_id: ClassID) -> Result<ClassInfo2, HRESULT>;
    fn get_code_info_2(&self, function_id: FunctionID) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT>;
    fn get_class_from_token_and_type_args(
        &self,
        module_id: ModuleID,
        type_def: mdTypeDef,
        type_args: Option<&[ClassID]>,
    ) -> Result<ClassID, HRESULT>;
    fn get_function_from_token_and_type_args(
        &self,
        module_id: ModuleID,
        func_def: mdMethodDef,
        class_id: ClassID,
        type_args: Option<&[ClassID]>,
    ) -> Result<FunctionID, HRESULT>;
    fn get_array_object_info(
        &self,
        object_id: ObjectID,
        dimensions: u32,
    ) -> Result<ArrayObjectInfo, HRESULT>;
    fn get_box_class_layout(&self, class_id: ClassID) -> Result<u32, HRESULT>;
    fn get_thread_app_domain(&self, thread_id: ThreadID) -> Result<AppDomainID, HRESULT>;
    fn get_rva_static_address(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
    ) -> Result<*const c_void, HRESULT>;
    fn get_app_domain_static_address(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        app_domain_id: AppDomainID,
    ) -> Result<*const c_void, HRESULT>;
    fn get_thread_static_address(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        thread_id: ThreadID,
    ) -> Result<*const c_void, HRESULT>;
    fn get_context_static_address(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        context_id: ContextID,
    ) -> Result<*const c_void, HRESULT>;
    fn get_static_field_info(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
    ) -> Result<COR_PRF_STATIC_TYPE, HRESULT>;
    fn get_generation_bounds(&self) -> Result<Vec<COR_PRF_GC_GENERATION_RANGE>, HRESULT>;
    fn get_object_generation(
        &self,
        object_id: ObjectID,
    ) -> Result<COR_PRF_GC_GENERATION_RANGE, HRESULT>;
    fn get_notified_exception_clause_info(&self) -> Result<COR_PRF_EX_CLAUSE_INFO, HRESULT>;
}
