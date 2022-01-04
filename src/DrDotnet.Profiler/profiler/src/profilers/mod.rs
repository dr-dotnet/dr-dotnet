pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

use profiling_api::{
    ClrProfiler,
    CorProfilerCallback9
};

use profiling_api::ffi::{
    ClassFactory as FFIClassFactory,
    CorProfilerCallback as FFICorProfilerCallback,
    E_FAIL as FFI_E_FAIL,
    GUID as FFI_GUID,
    HRESULT as FFI_HRESULT,
    LPVOID as FFI_LPVOID,
    REFCLSID as FFI_REFCLSID,
    REFIID as FFI_REFIID
};

unsafe fn is_guid_matching<T: ClrProfiler>(rclsid: FFI_REFCLSID) -> bool {
    let clsid = FFI_GUID::from(T::get_guid());
    return *rclsid == clsid;
}

unsafe fn try_attach<T: Clone + CorProfilerCallback9>(riid: FFI_REFIID, ppv: *mut FFI_LPVOID) -> FFI_HRESULT {
    let profiler = T::new();
    let class_factory: &mut FFIClassFactory<T> = FFIClassFactory::new(profiler);
    class_factory.QueryInterface(riid, ppv)
}

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: FFI_REFCLSID, riid: FFI_REFIID, ppv: *mut FFI_LPVOID) -> FFI_HRESULT {

    println!("[profiler] Entered DllGetClassObject");

    if ppv.is_null() {
        return FFI_E_FAIL;
    }

    let mut profiler_found = false;
    profiler_found = is_guid_matching::<ExceptionsProfiler>(rclsid);
    if profiler_found {
        return try_attach::<ExceptionsProfiler>(riid, ppv);
    }

    return FFI_E_FAIL;
}