use crate::{
    ffi::{ModuleID, BYTE, HRESULT},
    CorProfilerInfo6,
};

pub trait CorProfilerInfo7: CorProfilerInfo6 {
    fn apply_metadata(&self, module_id: ModuleID) -> Result<(), HRESULT>;
    fn get_in_memory_symbols_length(&self, module_id: ModuleID) -> Result<u32, HRESULT>;
    fn read_in_memory_symbols(
        &self,
        module_id: ModuleID,
        symbols_read_offset: u32,
        count_symbol_bytes: u32,
    ) -> Result<Vec<BYTE>, HRESULT>;
}
