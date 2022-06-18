#![allow(unused_variables)]
use crate::{
    ffi::{
        GCHandleID, ObjectID, ThreadID, BOOL, COR_PRF_FINALIZER_FLAGS, COR_PRF_GC_REASON,
        COR_PRF_GC_ROOT_FLAGS, COR_PRF_GC_ROOT_KIND, HRESULT, UINT_PTR,
    },
    CorProfilerCallback,
};

pub trait CorProfilerCallback2: CorProfilerCallback {
    fn thread_name_changed(&mut self, thread_id: ThreadID, name: &str) -> Result<(), HRESULT> {
        Ok(())
    }

    fn garbage_collection_started(
        &mut self,
        generation_collected: &[BOOL],
        reason: COR_PRF_GC_REASON,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn surviving_references(
        &mut self,
        object_id_range_start: &[ObjectID],
        object_id_range_length: &[u32], // TODO: Maybe make actual tuple. Simple zip-op.
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), HRESULT> {
        Ok(())
    }

    fn finalizeable_object_queued(
        &mut self,
        finalizer_flags: COR_PRF_FINALIZER_FLAGS,
        object_id: ObjectID,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn root_references_2(
        &mut self,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        root_ids: &[UINT_PTR], // TODO: Maybe this should be a single array of some struct kind.
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn handle_created(
        &mut self,
        handle_id: GCHandleID,
        initial_object_id: ObjectID,
    ) -> Result<(), HRESULT> {
        Ok(())
    }

    fn handle_destroyed(&mut self, handle_id: GCHandleID) -> Result<(), HRESULT> {
        Ok(())
    }
}
