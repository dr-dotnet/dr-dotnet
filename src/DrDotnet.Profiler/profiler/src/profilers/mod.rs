pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

use profiling_api::*;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilerData {
    pub profiler_id: Uuid,
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
        pub unsafe fn attach(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT {
            $(
                let clsid = ffi::GUID::from(<$type>::get_info().profiler_id);
                if *rclsid == clsid {
                    let profiler = <$type>::new();
                    let class_factory: &mut ffi::ClassFactory<$type> = ffi::ClassFactory::new(profiler);
                    return class_factory.QueryInterface(riid, ppv)
                }
            )+
            return ffi::E_FAIL;
        }
        pub fn get_profiler_infos() -> [ProfilerData; count!($($type)*)] {
            return [$(<$type>::get_info(),)+]
        }
    )
}

register!(ExceptionsProfiler);

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT {

    println!("[profiler] Entered DllGetClassObject");

    if ppv.is_null() {
        return ffi::E_FAIL;
    }

    return attach(rclsid, riid, ppv);
}