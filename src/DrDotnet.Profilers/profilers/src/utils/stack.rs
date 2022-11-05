use profiling_api::*;

pub unsafe extern "system" fn stack_snapshot_callback(method_id: usize, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *const libc::c_void) -> i32 {
    let client_data = client_data as *mut StackData;
    let info = &(*client_data).profiler_info;
    let stacktrace = &mut (*client_data).stacktrace;
    stacktrace.push(extensions::get_method_name(info, method_id)); // Not the most optimal way since it will pre allocate on every string
    return 0;
}

pub unsafe extern "system" fn stack_snapshot_callback2(method_id: usize, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *const libc::c_void) -> i32 {
    warn!("method id: {}", method_id);
    let mut data = Box::from_raw(client_data as *mut Vec<usize>);
    // let client_data = client_data as *mut Vec<usize>;
    // let client_data = &mut *client_data;
    // client_data.push(method_id);
    data.push(method_id);
    return 0;
}

pub struct StackData {
    pub profiler_info: ProfilerInfo,
    pub stacktrace: Vec<String>
}