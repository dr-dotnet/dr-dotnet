use std::collections::HashMap;
use profiling_api::*;
use uuid::Uuid;

use crate::report::*;
use crate::profilers::*;

#[derive(Default, Clone)]
pub struct MemoryLeakProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    surviving_references: HashMap<String, u64>,
    collections: u64,
}

impl Profiler for MemoryLeakProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E83").unwrap(),
            name: "Memory Leak Finder".to_owned(),
            description: "Look for managed memory leaks".to_owned(),
            is_released: true,
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl CorProfilerCallback for MemoryLeakProfiler {}

impl CorProfilerCallback2 for MemoryLeakProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections += 1;

        Ok(())
    }
}

impl CorProfilerCallback3 for MemoryLeakProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);

        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        match init_session(client_data, client_data_length) {
            Ok(uuid) => {
                self.session_id = uuid;
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        if self.profiler_info().force_gc().is_err() {
            error!("Force GC failed");
        }
        
        detach_after_duration::<MemoryLeakProfiler>(&self, 60, None);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, MemoryLeakProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        // report.write_line(format!("# Memory Leak Report"));
        // report.write_line(format!("## Total Collections"));
        // report.write_line(format!("**Total Collections**: {}", self.collections));
        // report.write_line(format!("## Surviving References by Class"));

        // use itertools::Itertools;

        // for surviving_reference in self.surviving_references.iter().sorted_by_key(|x| -(*x.1 as i128)) {
        //     report.write_line(format!("- {}: {}", surviving_reference.0, surviving_reference.1));
        // }

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for MemoryLeakProfiler
{
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_id_range_start: &[ffi::ObjectID], c_object_id_range_length: &[usize]) -> Result<(), ffi::HRESULT>
    {
        Ok(())
    }
}

impl CorProfilerCallback5 for MemoryLeakProfiler {}
impl CorProfilerCallback6 for MemoryLeakProfiler {}
impl CorProfilerCallback7 for MemoryLeakProfiler {}
impl CorProfilerCallback8 for MemoryLeakProfiler {}
impl CorProfilerCallback9 for MemoryLeakProfiler {}