use crate::api::ffi::*;
use crate::api::*;

pub trait StackSnapshotCallbackReceiver {
    type AssociatedType: StackSnapshotCallbackReceiver;

    fn callback(&mut self, method_id: usize, ip: usize, frame_info: usize, context: &[u8]);

    fn do_stack_snapshot(&mut self, pinfo: ClrProfilerInfo, thread_id: usize, use_context: bool) {
        let _ = pinfo.do_stack_snapshot(
            thread_id,
            Self::stack_snapshot_callback,
            if use_context {
                COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_REGISTER_CONTEXT
            } else {
                COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT
            },
            self as *mut Self as *mut std::ffi::c_void,
            std::ptr::null(),
            0,
        );
    }

    unsafe extern "system" fn stack_snapshot_callback(
        method_id: FunctionID,
        ip: usize,
        frame_info: usize,
        context_size: u32,
        context: *const u8,
        client_data: *mut std::ffi::c_void,
    ) -> HRESULT {
        let temp_vec;
        let context_slice: &[u8] = if context_size > 0 {
            std::slice::from_raw_parts(context, context_size as usize)
        } else {
            temp_vec = Vec::<u8>::new();
            &temp_vec
        };
        let receiver_ptr = client_data as *mut Self::AssociatedType;
        let receiver = &mut *receiver_ptr;
        receiver.callback(method_id, ip, frame_info, context_slice);
        return HRESULT::S_OK;
    }
}
