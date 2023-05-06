use crate::api::ffi::*;

pub unsafe extern "system" fn stack_snapshot_callback(method_id: FunctionID, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *mut libc::c_void) -> HRESULT {
    let vec = &mut *client_data.cast::<Vec<usize>>();
    vec.push(method_id);
    return HRESULT::S_OK;
}

pub unsafe extern "system" fn enum_references_callback(root: ObjectID, reference: *const ObjectID, client_data: *mut libc::c_void) -> BOOL {
    let vec = &mut *client_data.cast::<Vec<ObjectID>>();
    vec.push(*reference);
    return 1;
}