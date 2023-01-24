#![allow(unused_variables)]
use crate::{
    ffi::{
        mdMethodDef, CorProfilerFunctionControl, FunctionID, ModuleID, ObjectID, ReJITID, HRESULT,
    },
    CorProfilerCallback3,
};

pub trait CorProfilerCallback4: CorProfilerCallback3 {
    fn rejit_compilation_started(
        &mut self,
        function_id: FunctionID,
        rejit_id: ReJITID,
        is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn get_rejit_parameters(
        &mut self,
        module_id: ModuleID,
        method_id: mdMethodDef,
        function_control: &CorProfilerFunctionControl,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn rejit_compilation_finished(
        &mut self,
        function_id: FunctionID,
        rejit_id: ReJITID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
        is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn rejit_error(
        &mut self,
        module_id: ModuleID,
        method_id: mdMethodDef,
        function_id: FunctionID,
        hr_status: HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn moved_references_2(
        &mut self,
        old_object_ids: &[ObjectID],
        new_object_ids: &[ObjectID],
        object_lengths: &[usize],
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn surviving_references_2(
        &mut self,
        object_ids: &[ObjectID],
        object_lengths: &[usize],
    ) -> Result<(), HRESULT> {
        Ok(())
    }
}
