pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

use profiling_api::{
    ClrProfiler,
    CorProfilerCallback9
};

use profiling_api::ffi::{
    ClassFactory as FFIClassFactory,
    E_FAIL as FFI_E_FAIL,
    GUID as FFI_GUID,
    HRESULT as FFI_HRESULT,
    LPVOID as FFI_LPVOID,
    REFCLSID as FFI_REFCLSID,
    REFIID as FFI_REFIID
};

use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfilerData {
    pub guid: Uuid,
    pub name: String,
    pub description: String,
}

pub trait Profiler {
    fn get_info() -> ProfilerData;
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! register{
    ($($type:ty),+) => (
        pub unsafe fn attach(rclsid: FFI_REFCLSID, riid: FFI_REFIID, ppv: *mut FFI_LPVOID) -> FFI_HRESULT {
            $(
                let clsid = FFI_GUID::from(<$type>::get_info().guid);
                if *rclsid == clsid {
                    let profiler = <$type>::new();
                    let class_factory: &mut FFIClassFactory<$type> = FFIClassFactory::new(profiler);
                    return class_factory.QueryInterface(riid, ppv)
                }
            )+
            return FFI_E_FAIL;
        }
        pub fn get_profiler_infos() -> [ProfilerData; count!($($type)*)] {
            return [$(<$type>::get_info(),)+]
        }
    )
}

register!(ExceptionsProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: FFI_REFCLSID, riid: FFI_REFIID, ppv: *mut FFI_LPVOID) -> FFI_HRESULT {

    println!("[profiler] Entered DllGetClassObject");

    if ppv.is_null() {
        return FFI_E_FAIL;
    }

    return attach(rclsid, riid, ppv);
}