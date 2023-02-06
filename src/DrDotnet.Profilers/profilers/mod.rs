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

use uuid::Uuid;
use serde::{Deserialize, Serialize};

use simplelog::*;
use std::fs::File;

use crate::api::*;
use crate::report::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilerData {
    pub profiler_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_released: bool,
}

pub trait Profiler : CorProfilerCallback9 {
    fn get_info() -> ProfilerData;
    fn profiler_info(&self) -> &ProfilerInfo;
}

pub fn detach_after_duration<T: Profiler>(profiler: &T, duration_seconds: u64, callback: Option<Box<dyn Fn() + Send>>)
{
    let profiler_info = profiler.profiler_info().clone();

    std::thread::spawn(move || {
        if (thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).is_err()) {
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
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(format!("{}/profiler.debug.log", Session::get_root_directory())).unwrap()),
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

pub fn init_session(data: *const std::os::raw::c_void, data_length: u32) -> Result<Uuid, ffi::HRESULT> {
    unsafe {
        if data_length <= 0 {
            error!("Data should be non empty to carry the session ID");
            return Err(ffi::E_FAIL);
        }
        let cstr = std::ffi::CStr::from_ptr(data as *const _).to_string_lossy();
        match Uuid::parse_str(&cstr) {
            Ok(uuid) => {
                info!("Successfully parsed session ID {}", uuid);
                Ok(uuid)
            },
            Err(_) => {
                error!("Failed to parse session ID from FFI data");
                Err(ffi::E_FAIL)
            }
        }
    }
}