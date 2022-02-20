mod profilers;
mod report;
mod interop;
mod macros;

#[macro_use]
extern crate log;

// All profilers registered
register!(ExceptionsProfiler, AllocationByClassProfiler, MemoryLeakProfiler, RuntimePauseProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT
{
    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");

    println!("[profiler] Entered DllGetClassObject");
    if ppv.is_null() {
        return ffi::E_FAIL;
    }
    return attach(rclsid, riid, ppv);
}