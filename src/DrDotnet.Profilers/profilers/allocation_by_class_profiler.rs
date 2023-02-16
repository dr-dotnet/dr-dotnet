use dashmap::DashMap;
use std::sync::atomic::{AtomicIsize, Ordering};

use crate::api::*;
use crate::report::*;
use crate::profilers::*;

#[derive(Default)]
pub struct AllocationByClassProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_info: SessionInfo,
    allocations_by_class: DashMap<String, AtomicIsize>,
    collections: usize,
}

impl Profiler for AllocationByClassProfiler {

    fn get_info() -> ProfilerMetadata {
        return ProfilerMetadata {
            uuid: "805A308B-061C-47F3-9B30-F785C3186E84".to_owned(),
            name: "Allocations by Class".to_owned(),
            description: "For now, just allocations by class".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl CorProfilerCallback for AllocationByClassProfiler
{
    fn objects_allocated_by_class(&mut self, class_ids: &[ffi::ClassID], num_objects: &[u32]) -> Result<(), ffi::HRESULT>
    {
        // TODO: https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo10-enumerateobjectreferences-method
        for i in 0..class_ids.len() {
            
            let pinfo = self.profiler_info();
            let name = match pinfo.get_class_id_info(class_ids[i]) {
                Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                _ => "unknown2".to_owned()
            };

            let key = name;
            let count = num_objects[i] as isize;

            match self.allocations_by_class.get_mut(&key) {
                Some(pair) => { pair.value().fetch_add(count, Ordering::Relaxed); },
                None => { self.allocations_by_class.insert(key, AtomicIsize::new(count)); },
            }
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for AllocationByClassProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections += 1;

        Ok(())
    }
}

impl CorProfilerCallback3 for AllocationByClassProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.profiler_info = Some(profiler_info);
        
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        match init_session(client_data, client_data_length) {
            Ok(s) => {
                self.session_info = s;
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        detach_after_duration::<AllocationByClassProfiler>(&self, 10, None);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let mut report = self.session_info.create_report("summary.md".to_owned());

        report.write_line(format!("# Allocations Report"));
        report.write_line(format!("## Total Collections"));
        report.write_line(format!("**Total Collections**: {}", self.collections));
        report.write_line(format!("## Allocations by Class"));

        use itertools::Itertools;

        for allocations_for_class in self.allocations_by_class.iter().sorted_by_key(|x| -x.value().load(Ordering::Relaxed)) {
            report.write_line(format!("- {}: {}", allocations_for_class.key(), allocations_for_class.value().load(Ordering::Relaxed)));
        }

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for AllocationByClassProfiler {}
impl CorProfilerCallback5 for AllocationByClassProfiler {}
impl CorProfilerCallback6 for AllocationByClassProfiler {}
impl CorProfilerCallback7 for AllocationByClassProfiler {}
impl CorProfilerCallback8 for AllocationByClassProfiler {}
impl CorProfilerCallback9 for AllocationByClassProfiler {}