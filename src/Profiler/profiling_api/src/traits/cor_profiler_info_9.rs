use crate::{
    ffi::{FunctionID, ReJITID, COR_DEBUG_IL_TO_NATIVE_MAP, COR_PRF_CODE_INFO, HRESULT, UINT_PTR},
    CorProfilerInfo8,
};

pub trait CorProfilerInfo9: CorProfilerInfo8 {
    fn get_native_code_start_addresses(
        &self,
        function_id: FunctionID,
        rejit_id: ReJITID,
    ) -> Result<Vec<UINT_PTR>, HRESULT>;
    fn get_il_to_native_mapping_3(
        &self,
        native_code_start_address: UINT_PTR,
    ) -> Result<Vec<COR_DEBUG_IL_TO_NATIVE_MAP>, HRESULT>;
    fn get_code_info_4(
        &self,
        native_code_start_address: UINT_PTR,
    ) -> Result<Vec<COR_PRF_CODE_INFO>, HRESULT>;
}
