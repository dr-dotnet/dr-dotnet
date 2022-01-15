pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

pub mod memory_leak_profiler;
pub use memory_leak_profiler::MemoryLeakProfiler as MemoryLeakProfiler;

pub mod allocation_by_class_profiler;
pub use allocation_by_class_profiler::AllocationByClassProfiler as AllocationByClassProfiler;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use profiling_api::*;

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