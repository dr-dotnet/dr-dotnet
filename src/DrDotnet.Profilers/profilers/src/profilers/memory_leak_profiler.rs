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
            is_released: false,
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
        
        detach_after_duration::<MemoryLeakProfiler>(&self, 10);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, MemoryLeakProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        report.write_line(format!("# Memory Leak Report"));
        report.write_line(format!("## Total Collections"));
        report.write_line(format!("**Total Collections**: {}", self.collections));
        report.write_line(format!("## Surviving References by Class"));

        use itertools::Itertools;

        for surviving_reference in self.surviving_references.iter().sorted_by_key(|x| -(*x.1 as i128)) {
            report.write_line(format!("- {}: {}", surviving_reference.0, surviving_reference.1));
        }

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for MemoryLeakProfiler
{
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_id_range_start: &[ffi::ObjectID], c_object_id_range_length: &[usize]) -> Result<(), ffi::HRESULT>
    {
        fn get_inner_type(info: &ProfilerInfo, class_id: usize, array_dimension: &mut usize) -> usize {
            // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
            match info.is_array_class(class_id) {
                Ok(array_class_info) => {
                    *array_dimension = *array_dimension + 1;
                    // TODO: Handle array_class_info.rank
                    get_inner_type(info, array_class_info.element_class_id.unwrap(), array_dimension)
                },
                Err(_) => class_id,
            }
        }

        // TODO: https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getobjectgeneration-method
        // Use this to track new but long living objects

        for i in 0..object_id_range_start.len()
        {
            let mut array_dimension = 0;
            let pinfo = self.profiler_info();
            let mut key = match pinfo.get_class_from_object(object_id_range_start[i]) {
                Ok(class_id) => {
                    let class_id = get_inner_type(pinfo, class_id, &mut array_dimension);
                    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
                    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
                    match pinfo.get_class_id_info(class_id) {
                        Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                        _ => "unknown2".to_owned()
                    }
                }
                _ => "unknown1".to_owned()
            };

            if array_dimension > 0 {
                let mut brackets = String::with_capacity(array_dimension);
                for _ in 0..array_dimension {
                    brackets.push_str("[]");
                }
                key.push_str(&brackets);
                // let size = pinfo.get_object_size_2(object_id_range_start[i]).unwrap();
                // let s = format!("({})", size);
                // key.push_str(&s);
            }

            let value = 1; // c_object_id_range_length[i] as u64;

            *self.surviving_references.entry(key).or_insert(1) += 1;

            // match self.surviving_references.get_mut(&key) {
            //     Some(pair) => { *pair += value; },
            //     None => { self.surviving_references.insert(key, value); },
            // }
        }

        Ok(())
    }
}

impl CorProfilerCallback5 for MemoryLeakProfiler {}
impl CorProfilerCallback6 for MemoryLeakProfiler {}
impl CorProfilerCallback7 for MemoryLeakProfiler {}
impl CorProfilerCallback8 for MemoryLeakProfiler {}
impl CorProfilerCallback9 for MemoryLeakProfiler {}