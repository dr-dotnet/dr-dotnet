pub unsafe extern "system" fn stack_snapshot_callback(method_id: usize, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *const libc::c_void) -> i32 {
    let client_data = client_data as *mut StackData;
    let info = &(*client_data).profiler_info;
    let stacktrace = &mut (*client_data).stacktrace;
    stacktrace.push(extensions::get_method_name(info, method_id)); // Not the most optimal way since it will pre allocate on every string
    return 0;
}

pub struct StackData {
    profiler_info: ProfilerInfo,
    stacktrace: Vec<String>
}

// let sdt = StackData { profiler_info: pinfo.clone(), stacktrace: vec![] };
// let sd = &sdt as *const StackData;
// let sd = sd as *const std::ffi::c_void;

// pinfo.do_stack_snapshot(0, stack_snapshot_callback, ffi::COR_PRF_SNAPSHOT_INFO::COR_PRF_SNAPSHOT_DEFAULT, sd, std::ptr::null(), 0).ok();