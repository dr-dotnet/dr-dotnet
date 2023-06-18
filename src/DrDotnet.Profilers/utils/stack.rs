use crate::api::ffi::HRESULT;

pub unsafe extern "system" fn stack_snapshot_callback(method_id: usize, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *mut libc::c_void) -> HRESULT {
    if method_id != 0 {
        let vec = &mut *client_data.cast::<Vec<usize>>();
        vec.push(method_id);
    }
    return HRESULT::S_OK;
}