#![allow(unused_variables)]
use crate::{
    ffi::{
        AppDomainID, AssemblyID, ClassID, FunctionID, ModuleID, ObjectID, ThreadID,
        COR_PRF_JIT_CACHE, COR_PRF_SUSPEND_REASON, COR_PRF_TRANSITION_REASON, DWORD, GUID, HRESULT,
        REFGUID, UINT_PTR,
    },
    traits::ClrProfiler,
    ProfilerInfo,
};
use std::ffi::c_void;

pub trait CorProfilerCallback: ClrProfiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn app_domain_creation_started(&mut self, app_domain_id: AppDomainID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn app_domain_creation_finished(
        &mut self,
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn app_domain_shutdown_started(&mut self, app_domain_id: AppDomainID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn app_domain_shutdown_finished(
        &mut self,
        app_domain_id: AppDomainID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn assembly_load_started(&mut self, assembly_id: AssemblyID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn assembly_load_finished(
        &mut self,
        assembly_id: AssemblyID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn assembly_unload_started(&mut self, assembly_id: AssemblyID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn assembly_unload_finished(
        &mut self,
        assembly_id: AssemblyID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_load_started(&mut self, module_id: ModuleID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn module_load_finished(
        &mut self,
        module_id: ModuleID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_unload_started(&mut self, module_id: ModuleID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn module_unload_finished(
        &mut self,
        module_id: ModuleID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn module_attached_to_assembly(
        &mut self,
        module_id: ModuleID,
        assembly_id: AssemblyID,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn class_load_started(&mut self, class_id: ClassID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn class_load_finished(
        &mut self,
        class_id: ClassID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn class_unload_started(&mut self, class_id: ClassID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn class_unload_finished(
        &mut self,
        class_id: ClassID,
        hr_status: HRESULT,
    ) -> Result<(), HRESULT> {
        // TODO: Create enum that actual encodes possible statuses instead of hresult param
        Ok(())
    }

    fn function_unload_started(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_cached_function_search_started(
        &mut self,
        function_id: FunctionID,
        use_cached_function: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_cached_function_search_finished(
        &mut self,
        function_id: FunctionID,
        result: COR_PRF_JIT_CACHE,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_function_pitched(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn jit_inlining(
        &mut self,
        caller_id: FunctionID,
        callee_id: FunctionID,
        should_inline: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn thread_created(&mut self, thread_id: ThreadID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn thread_destroyed(&mut self, thread_id: ThreadID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn thread_assigned_to_os_thread(
        &mut self,
        managed_thread_id: ThreadID,
        os_thread_id: DWORD,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_client_invocation_started(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_client_sending_message(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_client_receiving_reply(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_client_invocation_finished(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_server_receiving_message(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_server_invocation_started(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_server_invocation_returned(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn remoting_server_sending_reply(
        &mut self,
        cookie: GUID,
        is_async: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn unmanaged_to_managed_transition(
        &mut self,
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn managed_to_unmanaged_transition(
        &mut self,
        function_id: FunctionID,
        reason: COR_PRF_TRANSITION_REASON,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_suspend_started(
        &mut self,
        suspend_reason: COR_PRF_SUSPEND_REASON,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_suspend_finished(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_suspend_aborted(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_resume_started(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_resume_finished(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_thread_suspended(&mut self, thread_id: ThreadID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn runtime_thread_resumed(&mut self, thread_id: ThreadID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn moved_references(
        &mut self,
        old_object_id_range_start: &[ObjectID],
        new_object_id_range_start: &[ObjectID],
        object_id_range_length: &[u32],
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn object_allocated(&mut self, object_id: ObjectID, class_id: ClassID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn objects_allocated_by_class(
        &mut self,
        class_ids: &[ClassID],
        num_objects: &[u32],
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn object_references(
        &mut self,
        object_id: ObjectID,
        class_id: ClassID,
        object_ref_ids: &[ObjectID],
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn root_references(&mut self, root_ref_ids: &[ObjectID]) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_thrown(&mut self, thrown_object_id: ObjectID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_search_function_enter(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_search_function_leave(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_search_filter_enter(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_search_filter_leave(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_search_catcher_found(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_os_handler_enter(&mut self, _unused: UINT_PTR) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_os_handler_leave(&mut self, _unused: UINT_PTR) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_unwind_function_enter(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_unwind_function_leave(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_unwind_finally_enter(&mut self, function_id: FunctionID) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_unwind_finally_leave(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_catcher_enter(
        &mut self,
        function_id: FunctionID,
        object_id: ObjectID,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_catcher_leave(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn com_classic_vtable_created(
        &mut self,
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: *const c_void,
        c_slots: u32,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn com_classic_vtable_destroyed(
        &mut self,
        wrapped_class_id: ClassID,
        implemented_iid: REFGUID,
        p_vtable: *const c_void,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_clr_catcher_found(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn exception_clr_catcher_execute(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }
}
