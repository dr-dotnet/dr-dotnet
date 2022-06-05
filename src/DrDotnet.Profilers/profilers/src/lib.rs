mod profilers;
mod report;
mod interop;
mod macros;

#[macro_use]
extern crate log;

// Create function to list and attach profilers
register!(ExceptionsProfiler, AllocationByClassProfiler, MemoryLeakProfiler, RuntimePauseProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT
{
    debug!("[profiler] Entered DllGetClassObject");

    if ppv.is_null() {
        return ffi::E_FAIL;
    }
    return attach(rclsid, riid, ppv);
}