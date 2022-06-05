use std::borrow::BorrowMut;
use std::ops::Add;

use std::collections::HashMap;
use profiling_api::*;
use uuid::Uuid;

use crate::report::*;
use crate::profilers::*;

pub struct GCSurvivorsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    surviving_references: HashMap<String, u64>,
    collections: u64,
}

impl Profiler for GCSurvivorsProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E86").unwrap(),
            name: "GC Survivors".to_owned(),
            description: "Lists objects that survived a garbage collection".to_owned(),
            isReleased: true,
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl Clone for GCSurvivorsProfiler {
    fn clone(&self) -> Self { 
        GCSurvivorsProfiler {
            profiler_info: self.profiler_info.clone(),
            session_id: self.session_id.clone(),
            surviving_references: HashMap::new(),
            collections: 0
        }
    }
}

impl ClrProfiler for GCSurvivorsProfiler {
    fn new() -> GCSurvivorsProfiler {
        GCSurvivorsProfiler {
            profiler_info: None,
            session_id: Uuid::default(),
            surviving_references: HashMap::new(),
            collections: 0
        }
    }
}

impl CorProfilerCallback for GCSurvivorsProfiler {}

impl CorProfilerCallback2 for GCSurvivorsProfiler
{
    /*
    fn surviving_references(&mut self, object_id_range_start: &[ffi::ObjectID], object_id_range_length: &[u32]) -> Result<(), ffi::HRESULT>
    {
        for i in 0..object_id_range_start.len()
        {
            let pinfo = self.profiler_info();
            let name = 
            match pinfo.get_class_from_object(object_id_range_start[i]) {
                Ok(class_id) => 
                match pinfo.get_class_id_info(class_id) {
                    Ok(class_info) => extensions::get_type_name(pinfo, class_info.module_id, class_info.token),
                    _ => "unknown2".to_owned()
                },
                _ => "unknown1".to_owned()
            };
    
            let key = name;
            let value = object_id_range_length[i] as u64;
            match self.surviving_references.get_mut(&key) {
                Some(pair) => { pair.value().add(value); },
                None => { self.surviving_references.insert(key, value); },
            }
        }

        Ok(())
    }
    */

    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections += 1;

        Ok(())
    }
}

impl CorProfilerCallback3 for GCSurvivorsProfiler
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
        
        detach_after_duration::<GCSurvivorsProfiler>(&self, 10);
        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, GCSurvivorsProfiler::get_info());

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

impl CorProfilerCallback4 for GCSurvivorsProfiler
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

impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}