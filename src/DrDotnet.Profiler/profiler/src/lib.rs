mod profilers;
mod report;
mod interop;
mod macros;

// All profilers registered
register!(ExceptionsProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT
{
    println!("[profiler] Entered DllGetClassObject");
    if ppv.is_null() {
        return ffi::E_FAIL;
    }
    return attach(rclsid, riid, ppv);
}