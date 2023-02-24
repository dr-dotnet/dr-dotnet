mod profilers;
mod session;
mod interop;
mod macros;
mod utils;
mod api;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;

mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

// Create function to list and attach profilers
register!(
    GCSurvivorsProfiler,
    ExceptionsProfiler,
    AllocationByClassProfiler,
    MemoryLeakProfiler,
    RuntimePauseProfiler,
    CpuHotpathProfiler,
    DuplicatedStringsProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT
{
    profilers::init_logging();

    debug!("DllGetClassObject(rclsid: {:?}, riid: {:?})", rclsid, riid);

    if ppv.is_null() {
        return ffi::HRESULT::E_FAIL;
    }
    
    return attach(rclsid, riid, ppv);
}