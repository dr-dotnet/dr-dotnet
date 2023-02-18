#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! register{
    ($($type:ty),+) => (

        use crate::api::*;
        use profilers::*;
        use crate::rust_protobuf_protos::interop::*;

        // Attaches the profiler with the given rclsid to the targeted process.
        pub unsafe fn attach(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT {
            $(
                let clsid = ffi::GUID::from(<$type>::profiler_info().uuid);
                if *rclsid == clsid {
                    let profiler = <$type>::default();
                    let class_factory : &mut ffi::ClassFactory<$type> = ffi::ClassFactory::new(profiler);
                    return class_factory.QueryInterface(riid, ppv)
                }
            )+
            error!("No matched profiler");
            return ffi::CLASS_E_CLASSNOTAVAILABLE;
        }

        // Returns the list of profilers that are registered, along with their information.
        // This function is called through PInvoke from the UI in order to list available profilers.
        pub fn get_profiler_infos() -> [ProfilerInfo; count!($($type)*)] {
            return [$(<$type>::profiler_info(),)+]
        }
    )
}

#[macro_export]
macro_rules! profiler_getset {
    {} => {
        fn clr(&self) -> &ClrProfilerInfo {
            &self.clr_profiler_info
        }

        fn set_clr_profiler_info(&mut self, clr_profiler_info: &ClrProfilerInfo) {
            self.clr_profiler_info = clr_profiler_info.clone();
        }
    
        fn session_info(&self) -> &SessionInfo {
            &self.session_info
        }

        fn set_session_info(&mut self, session_info: &SessionInfo) {
            self.session_info = session_info.clone();
        }
    }
}

pub(crate) use profiler_getset;