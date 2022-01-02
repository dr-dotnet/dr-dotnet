use crate::{
    ffi::{
        mdFieldDef, AppDomainID, ClassID, CorProfilerFunctionEnum, CorProfilerModuleEnum,
        FunctionEnter3, FunctionEnter3WithInfo, FunctionID, FunctionIDMapper2, FunctionLeave3,
        FunctionLeave3WithInfo, FunctionTailcall3, FunctionTailcall3WithInfo, ModuleID, ThreadID,
        COR_PRF_ELT_INFO, COR_PRF_FRAME_INFO, HRESULT,
    },
    CorProfilerInfo2, FunctionEnter3Info, FunctionLeave3Info, ModuleInfo2, RuntimeInfo,
    StringLayout,
};
use std::ffi::c_void;

pub trait CorProfilerInfo3: CorProfilerInfo2 {
    fn enum_jited_functions(&self) -> Result<&mut CorProfilerFunctionEnum, HRESULT>;
    fn request_profiler_detach(&self, expected_completion_milliseconds: u32)
        -> Result<(), HRESULT>;
    fn set_function_id_mapper_2(
        &self,
        func: FunctionIDMapper2,
        client_data: *const c_void,
    ) -> Result<(), HRESULT>;
    fn get_string_layout_2(&self) -> Result<StringLayout, HRESULT>;
    fn set_enter_leave_function_hooks_3(
        &self,
        func_enter_3: FunctionEnter3,
        func_leave_3: FunctionLeave3,
        func_tailcall_3: FunctionTailcall3,
    ) -> Result<(), HRESULT>;
    fn set_enter_leave_function_hooks_3_with_info(
        &self,
        func_enter_3_with_info: FunctionEnter3WithInfo,
        func_leave_3_with_info: FunctionLeave3WithInfo,
        func_tailcall_3_with_info: FunctionTailcall3WithInfo,
    ) -> Result<(), HRESULT>;
    fn get_function_enter_3_info(
        &self,
        function_id: FunctionID,
        elt_info: COR_PRF_ELT_INFO,
    ) -> Result<FunctionEnter3Info, HRESULT>;
    fn get_function_leave_3_info(
        &self,
        function_id: FunctionID,
        elt_info: COR_PRF_ELT_INFO,
    ) -> Result<FunctionLeave3Info, HRESULT>;
    fn get_function_tailcall_3_info(
        &self,
        function_id: FunctionID,
        elt_info: COR_PRF_ELT_INFO,
    ) -> Result<COR_PRF_FRAME_INFO, HRESULT>;
    fn enum_modules(&self) -> Result<&mut CorProfilerModuleEnum, HRESULT>;
    fn get_runtime_information(&self) -> Result<RuntimeInfo, HRESULT>;
    fn get_thread_static_address_2(
        &self,
        class_id: ClassID,
        field_token: mdFieldDef,
        app_domain_id: AppDomainID,
        thread_id: ThreadID,
    ) -> Result<*const c_void, HRESULT>;
    fn get_app_domains_containing_module(
        &self,
        module_id: ModuleID,
    ) -> Result<Vec<AppDomainID>, HRESULT>;
    fn get_module_info_2(&self, module_id: ModuleID) -> Result<ModuleInfo2, HRESULT>;
}
