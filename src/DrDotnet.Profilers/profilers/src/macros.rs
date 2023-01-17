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
        use once_cell::sync::Lazy;

        // Attaches the profiler with the given rclsid to the targeted process.
        pub unsafe fn attach(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT {
            $(
                let clsid = ffi::GUID::from(<$type>::get_info().profiler_id);
                if *rclsid == clsid {
                    // static profiler_s: Lazy<std::sync::Mutex<$type>> = Lazy::new(|| {
                    //     std::sync::Mutex::new(<$type>::default())
                    // });
                    static class_factory_s: Lazy<std::sync::Mutex<&mut ffi::ClassFactory<$type>>> = Lazy::new(|| {
                        info!("Creating ClassFactory singleton");
                        let p = <$type>::default();
                        let c : &mut ffi::ClassFactory<$type> = ffi::ClassFactory::new(p);
                        std::sync::Mutex::new(c)
                    });
                    //let profiler = <$type>::default();
                    
                    // https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iclassfactory-createinstance#remarks
                    //let class_factory : &mut ffi::ClassFactory<$type> = ffi::ClassFactory::new(profiler);
                    //let class_factory : &mut ffi::ClassFactory<$type> = ffi::ClassFactory::new(*profiler_s.lock().unwrap());
                    info!("Querying interface");
                    return class_factory_s.lock().unwrap().QueryInterface(riid, ppv)
                }
            )+
            info!("No matched profiler");
            return profiling_api::ffi::CLASS_E_CLASSNOTAVAILABLE;
        }

        // Returns the list of profilers that are registered, along with their information.
        // This function is called through PInvoke from the UI in order to list available profilers.
        pub fn get_profiler_infos() -> [ProfilerData; count!($($type)*)] {
            return [$(<$type>::get_info(),)+]
        }
    )
}