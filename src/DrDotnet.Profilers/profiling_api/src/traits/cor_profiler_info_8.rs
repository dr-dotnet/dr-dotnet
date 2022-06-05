use crate::{
    ffi::{FunctionID, HRESULT, LPCBYTE},
    CorProfilerInfo7, DynamicFunctionInfo, FunctionAndRejit,
};

pub trait CorProfilerInfo8: CorProfilerInfo7 {
    fn is_function_dynamic(&self, function_id: FunctionID) -> Result<bool, HRESULT>;
    fn get_function_from_ip_3(&self, ip: LPCBYTE) -> Result<FunctionAndRejit, HRESULT>;
    fn get_dynamic_function_info(
        &self,
        function_id: FunctionID,
    ) -> Result<DynamicFunctionInfo, HRESULT>;
}
