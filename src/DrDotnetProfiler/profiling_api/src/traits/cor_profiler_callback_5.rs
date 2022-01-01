#![allow(unused_variables)]
use crate::{
    ffi::{GCHandleID, ObjectID, HRESULT},
    CorProfilerCallback4,
};

pub trait CorProfilerCallback5: CorProfilerCallback4 {
    fn conditional_weak_table_element_references(
        &mut self,
        key_ref_ids: &[ObjectID],
        value_ref_ids: &[ObjectID],
        root_ids: &[GCHandleID],
    ) -> Result<(), HRESULT> {
        Ok(())
    }
}
