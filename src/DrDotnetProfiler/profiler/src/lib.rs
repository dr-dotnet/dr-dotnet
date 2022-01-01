mod profiler;

#[no_mangle]
pub extern "C" fn string_from_rust() -> *const i8 {
    let s = std::ffi::CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}