pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

pub mod memory_leak_profiler;
pub use memory_leak_profiler::MemoryLeakProfiler as MemoryLeakProfiler;

pub mod allocation_by_class_profiler;
pub use allocation_by_class_profiler::AllocationByClassProfiler as AllocationByClassProfiler;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use profiling_api::*;

use simplelog::*;
use std::fs::File;

use crate::report::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilerData {
    pub profiler_id: Uuid,
    pub name: String,
    pub description: String,
}

pub trait Profiler: CorProfilerCallback9 {
    fn get_info() -> ProfilerData;
    fn profiler_info(&self) -> &ProfilerInfo;
}

pub fn detach_after_duration<T: Profiler>(profiler: &T, duration_seconds: u64)
{
    let profiler_info = profiler.profiler_info().clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(duration_seconds));
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo3-requestprofilerdetach-method
        // https://github.com/Potapy4/dotnet-coreclr/blob/master/Documentation/Profiling/davbr-blog-archive/Profiler%20Detach.md#requestprofilerdetach
        profiler_info.request_profiler_detach(3000).ok();
    });
}

#[cfg(debug_assertions)]
fn init_logging(uuid: Uuid) {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(format!("{}/profiler.debug.log", Session::get_directory(uuid))).unwrap()),
        ]
    ).unwrap();
}

#[cfg(not(debug_assertions))]
fn init_logging(uuid: Uuid) {
    let config = ConfigBuilder::new().set_max_level(LevelFilter.Error).build();
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, config, TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, config,File::create(format!("{}/profiler.release.log", Session::get_directory(uuid))).unwrap()),
        ]
    ).unwrap();
}

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
                init_logging(uuid);
                info!("Successfully initialized logging");
                Ok(uuid)
            },
            Err(_) => {
                error!("Failed to parse session ID from FFI data");
                Err(ffi::E_FAIL)
            }
        }
    }
}