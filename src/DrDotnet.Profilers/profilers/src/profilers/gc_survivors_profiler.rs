// Workflow:
// On GC start, clear surviving objects
// Append surviving objects  https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
// On GC end, if there are surviving objects in map (not empty), 

use std::borrow::BorrowMut;
use std::ops::Add;

use std::collections::HashMap;
use profiling_api::*;
use profiling_api::ffi::ObjectID;
use uuid::Uuid;

use crate::report::*;
use crate::profilers::*;

pub struct GCSurvivorsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    surviving_references: HashMap<ObjectID, u64>,
    object_to_referencers: HashMap<ffi::ObjectID, Vec<ffi::ObjectID>>,
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
            object_to_referencers: HashMap::new(),
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
            object_to_referencers: HashMap::new(),
            collections: 0
        }
    }
}

impl CorProfilerCallback for GCSurvivorsProfiler
{
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), ffi::HRESULT>
    {
        if self.surviving_references.len() == 0 {
            return Ok(())
        }

        //info!("Collecting references...");

        // Create dependency tree, but from object to referencers, instead of object to its references.
        // This is usefull for being able to browse from any object back to its roots.
        for object_ref_id in object_ref_ids {
            match self.object_to_referencers.get_mut(&object_ref_id) {
                Some(referencers) => {
                    referencers.push(object_id);
                },
                None => {
                    self.object_to_referencers.insert(*object_ref_id, vec![object_id]);
                }
            }
        }

        Ok(())
    }
}

impl GCSurvivorsProfiler
{
    fn append_referencers(&self, info: &ProfilerInfo, object_id: ffi::ObjectID, name: &mut String, depth: u32)
    {
        if depth > 10 {
            return;
        } 

        match self.object_to_referencers.get(&object_id) {
            Some(referencers) => {
                for referencer in referencers {

                    let gen = info.get_object_generation(*referencer).unwrap();

                    if gen.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                        let refname = GCSurvivorsProfiler::get_object_class_name(info, *referencer);
                        name.push_str(format!("--> held by {} ({:?})", refname, gen.generation).as_str());
                    }

                    // Recursive
                    self.append_referencers(info, *referencer, name, depth + 1);
                }
            },
            None => {}
        }
    }

    fn get_inner_type(info: &ProfilerInfo, class_id: usize, array_dimension: &mut usize) -> usize
    {
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
        match info.is_array_class(class_id) {
            Ok(array_class_info) => {
                *array_dimension = *array_dimension + 1;
                // TODO: Handle array_class_info.rank
                GCSurvivorsProfiler::get_inner_type(info, array_class_info.element_class_id.unwrap(), array_dimension)
            },
            Err(_) => class_id,
        }
    }

    fn get_object_class_name(info: &ProfilerInfo, object_id: ffi::ObjectID) -> String
    {
        let mut array_dimension = 0;

        let mut name = match info.get_class_from_object(object_id) {
            Ok(class_id) => {
                // As the class could be an array, we recursively dig until we find the inner type that is not an array
                let class_id = GCSurvivorsProfiler::get_inner_type(info, class_id, &mut array_dimension);
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
                match info.get_class_id_info(class_id) {
                    Ok(class_info) => extensions::get_type_name(info, class_info.module_id, class_info.token),
                    _ => "unknown2".to_owned()
                }
            }
            _ => "unknown1".to_owned()
        };

        if array_dimension > 0 {
            let mut brackets = String::with_capacity(array_dimension * 2);
            for _ in 0..array_dimension {
                brackets.push_str("[]");
            }
            name.push_str(&brackets);
        }

        return name;
    }
}

impl CorProfilerCallback2 for GCSurvivorsProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        self.collections += 1;

        let mut c = 0;
        for gen in generation_collected {
            if *gen == 1 {
                c += 1;
            }
        }

        self.surviving_references.clear();
        self.object_to_referencers.clear();

        info!("GC started on gen {} for reason {:?}", c, reason);

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT>
    {
        info!("GC finished");

        if self.surviving_references.len() == 0 {
            return Ok(())
        }

        // Disable profiling to free some resources
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        info!("Surviving references to process: {}", self.surviving_references.len());

        for (object_id, number) in self.surviving_references.iter()
        {
            let pinfo = self.profiler_info();
            let mut name = GCSurvivorsProfiler::get_object_class_name(pinfo, *object_id);

            self.append_referencers(pinfo, *object_id, &mut name, 0);

            info!(">>> {}", name);
        }

        // We're done, we can detach :)
        let profiler_info = self.profiler_info().clone();
        profiler_info.request_profiler_detach(3000).ok();

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
        //info!("Collecting surviving references...");

        for object_id in object_id_range_start {
            *self.surviving_references.entry(*object_id).or_insert(0) += 1;
        }

        Ok(())
    }
}

impl CorProfilerCallback5 for GCSurvivorsProfiler
{
    fn conditional_weak_table_element_references(
            &mut self,
            key_ref_ids: &[ffi::ObjectID],
            value_ref_ids: &[ffi::ObjectID],
            root_ids: &[ffi::GCHandleID],
        ) -> Result<(), ffi::HRESULT> {
        
            info!("conditional_weak_table_element_references");

            Ok(())
    }
}

impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}