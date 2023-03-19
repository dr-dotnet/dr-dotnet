pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

pub mod memory_leak_profiler;
pub use memory_leak_profiler::MemoryLeakProfiler as MemoryLeakProfiler;

pub mod allocation_by_class_profiler;
pub use allocation_by_class_profiler::AllocationByClassProfiler as AllocationByClassProfiler;

pub mod runtime_pause_profiler;
pub use runtime_pause_profiler::RuntimePauseProfiler as RuntimePauseProfiler;

pub mod gc_survivors_profiler;
pub use gc_survivors_profiler::GCSurvivorsProfiler as GCSurvivorsProfiler;

pub mod cpu_hotpath_profiler;
pub use cpu_hotpath_profiler::CpuHotpathProfiler as CpuHotpathProfiler;

pub mod duplicated_strings_profiler;
pub use duplicated_strings_profiler::DuplicatedStringsProfiler as DuplicatedStringsProfiler;

pub mod merged_call_stacks_profiler;
pub mod pstacks_profiler;

pub use merged_call_stacks_profiler::MergedCallStacksProfiler as MergedCallStacksProfiler;

use simplelog::*;
use std::fs::File;

use crate::api::*;
use crate::rust_protobuf_protos::interop::*;

pub trait Profiler : CorProfilerCallback9 {
    fn profiler_info() -> ProfilerInfo;

    fn session_info(&self) -> &SessionInfo;
    fn set_session_info(&mut self, session_info: &SessionInfo);

    fn clr(&self) -> &ClrProfilerInfo;
    fn set_clr_profiler_info(&mut self, clr_profiler_info: &ClrProfilerInfo);

    fn init(&mut self, event: ffi::COR_PRF_MONITOR, high_event: Option<ffi::COR_PRF_HIGH_MONITOR>, clr_profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.set_clr_profiler_info(&clr_profiler_info);

        let high_event_s = match high_event {
            Some(e) => e,
            None => ffi::COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_NONE
        };

        match self.clr().set_event_mask_2(event, high_event_s) {
            Ok(_) => match SessionInfo::init(client_data, client_data_length) {
                Ok(s) => {
                    self.set_session_info(&s);
                    Ok(())
                },
                Err(err) => {
                    error!("{}", err);
                    Err(ffi::HRESULT::E_FAIL)
                }
            },
            Err(hresult) => {
                error!("Error setting event mask: {:?}", hresult);
                Err(hresult)
            } 
        }
    }
}

pub fn detach_after_duration<T: Profiler>(profiler: &T, duration_seconds: u64, callback: Option<Box<dyn Fn() + Send>>)
{
    let profiler_info = profiler.clr().clone();

    std::thread::spawn(move || {
        if thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).is_err() {
            error!("Could not increase thread priority for detach operation");
        }
        std::thread::sleep(std::time::Duration::from_secs(duration_seconds));

        if let Some(ref func) = callback {
            (func)();
        }

        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo3-requestprofilerdetach-method
        // https://github.com/Potapy4/dotnet-coreclr/blob/master/Documentation/Profiling/davbr-blog-archive/Profiler%20Detach.md#requestprofilerdetach
        profiler_info.request_profiler_detach(3000).ok();
    });
}

use std::sync::atomic::{AtomicBool, Ordering};

static mut LOGGING_INITIALIZED: AtomicBool = AtomicBool::new(false);

// #[cfg(debug_assertions)]
pub fn init_logging() {
    
    // Init once.
    unsafe {
        if LOGGING_INITIALIZED.swap(true, Ordering::Relaxed) {
            error!("Logging is already initialized!");
            return;
        }
    }

    match CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(format!("{}/profiler.debug.log", SessionInfo::get_root_directory())).unwrap()),
        ]
    ) {
        Ok(_) => info!("Logging initialized!"),
        Err(error) => println!("Logging initialization failed: {:?}", error)
    }
}

// #[cfg(not(debug_assertions))]
// fn init_logging(uuid: Uuid) {
//     let config = ConfigBuilder::new().set_max_level(LevelFilter::Error).build();
//     CombinedLogger::init(
//         vec![
//             TermLogger::new(LevelFilter::Warn, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
//             WriteLogger::new(LevelFilter::Warn, config.clone(), File::create(format!("{}/profiler.release.log", Session::get_directory(uuid))).unwrap()),
//         ]
//     ).unwrap();
// }