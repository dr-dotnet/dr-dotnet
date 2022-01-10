pub mod exceptions_profiler;
pub use exceptions_profiler::ExceptionsProfiler as ExceptionsProfiler;

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
}
