use crate::{
    ffi::{mdMethodDef, ModuleID, ObjectID, ObjectReferenceCallback, COR_PRF_REJIT_FLAGS, HRESULT},
    CorProfilerInfo9,
};
use std::ffi::c_void;

pub trait CorProfilerInfo10: CorProfilerInfo9 {
    fn enumerate_object_references(
        &self,
        object_id: ObjectID,
        callback: ObjectReferenceCallback,
        client_data: *const c_void,
    ) -> Result<(), HRESULT>;
    fn is_frozen_object(&self, object_id: ObjectID) -> Result<bool, HRESULT>;
    fn get_loh_object_size_threshold(&self) -> Result<u32, HRESULT>;
    fn request_rejit_with_inliners(
        &self,
        dw_rejit_flags: COR_PRF_REJIT_FLAGS,
        module_ids: &[ModuleID],
        method_ids: &[mdMethodDef], // TODO: Maybe we want the pairs to be actual tuples. Simple zip op.
    ) -> Result<(), HRESULT>;
    fn suspend_runtime(&self) -> Result<(), HRESULT>;
    fn resume_runtime(&self) -> Result<(), HRESULT>;
}
