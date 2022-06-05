#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! register{
    ($($type:ty),+) => (

        use profiling_api::*;
        use profilers::*;

        // Attaches the profiler with the given rclsid to the targeted process.
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

        // Returns the list of profilers that are registered, along with their information.
        // This function is called through PInvoke from the UI in order to list available profilers.
        pub fn get_profiler_infos() -> [ProfilerData; count!($($type)*)] {
            return [$(<$type>::get_info(),)+]
        }
    )
}