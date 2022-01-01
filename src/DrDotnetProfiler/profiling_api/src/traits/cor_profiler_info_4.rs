use crate::{
    ffi::{
        mdMethodDef, CorProfilerFunctionEnum, CorProfilerThreadEnum, FunctionID, ModuleID,
        ObjectID, ReJITID, COR_DEBUG_IL_TO_NATIVE_MAP, COR_PRF_CODE_INFO, HRESULT, LPCBYTE,
    },
    CorProfilerInfo3, FunctionAndRejit,
};

pub trait CorProfilerInfo4: CorProfilerInfo3 {
    fn enum_threads(&self) -> Result<&mut CorProfilerThreadEnum, HRESULT>;
    fn initialize_current_thread(&self) -> Result<(), HRESULT>;
    fn request_rejit(
        &self,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe make the pairs actual tuples? Simple zip op.
    ) -> Result<(), HRESULT>;
    fn request_revert(
        &self,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe make the pairs actual tuples? Simple zip op.
    ) -> Result<Vec<HRESULT>, HRESULT>;
    fn get_code_info_3(
        &self,
        function_id: FunctionID,
        rejit_id: ReJITID,
    ) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT>;
    fn get_function_from_ip_2(&self, ip: LPCBYTE) -> Result<FunctionAndRejit, HRESULT>;
    fn get_rejit_ids(&self, function_id: FunctionID) -> Result<Vec<ReJITID>, HRESULT>;
    fn get_il_to_native_mapping_2(
        &self,
        function_id: FunctionID,
        rejit_id: ReJITID,
    ) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT>;
    fn enum_jited_functions_2(&self) -> Result<&mut CorProfilerFunctionEnum, HRESULT>;
    fn get_object_size_2(&self, object_id: ObjectID) -> Result<usize, HRESULT>;
}
