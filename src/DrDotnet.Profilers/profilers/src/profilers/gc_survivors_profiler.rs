// Workflow:
// On GC start, clear surviving objects
// Append surviving objects  https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
// On GC end, if there are surviving objects in map (not empty), 

use std::collections::HashMap;
use profiling_api::*;
use profiling_api::ffi::ObjectID;
use uuid::Uuid;
use itertools::Itertools;

use crate::report::*;
use crate::profilers::*;

pub struct GCSurvivorsProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    surviving_references: HashMap<ObjectID, u64>,
    object_to_referencers: HashMap<ffi::ObjectID, Vec<ffi::ObjectID>>,
    serialized_survivor_branches: HashMap<String, u64>,
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
            serialized_survivor_branches: HashMap::new(),
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
            serialized_survivor_branches: HashMap::new(),
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
    pub fn append_referencers(&self, info: &ProfilerInfo, object_id: ffi::ObjectID, max_depth: i32) -> Vec<String>
    {
        let mut branches = Vec::new();

        self.append_referencers_recursive(info, object_id, &mut String::new(), -max_depth, &mut branches);

        return branches;
    }

    // Recursively drill through references until we find a gen 2 object.
    fn append_referencers_recursive(&self, info: &ProfilerInfo, object_id: ffi::ObjectID, branch: &mut String, depth: i32, branches: &mut Vec<String>)
    {
        let gen = info.get_object_generation(object_id).unwrap();

        let refname = GCSurvivorsProfiler::get_object_class_name(info, object_id);
        branch.push_str(refname.as_str());
        if gen.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
            branch.push_str(" (Gen 2)");
        }

        // Escape in case of circular references (could be done in a better way)
        if depth > 0 {
            // Only add branches that are rooted to gen 2
            if gen.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                branches.push(branch.clone());
            }
            return;
        }

        match self.object_to_referencers.get(&object_id) {
            Some(referencers) =>
            {
                let branch_current_len = branch.len();

                for i in 0..referencers.len()
                {
                    if i == 0
                    {
                        // Same branch, we keep on this same branch
                        branch.push_str(" < ");
                        self.append_referencers_recursive(info, referencers[0], branch, depth + 1, branches);
                    }
                    else
                    {
                        // New branch. We clone the current branch to append next holders
                        let mut branch_copy = branch[..branch_current_len].to_string();
                        branch_copy.push_str(" < ");
                        self.append_referencers_recursive(info, referencers[i], &mut branch_copy, depth + 1, branches);
                    }
                }
            },
            None => {
                // Only add branches that are rooted to gen 2
                if gen.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                    branches.push(branch.clone());
                }
            }
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
        // Data from previous garbage collections are no longer valid, so we clear it when a new garbage collection starts.
        self.surviving_references.clear();
        self.object_to_referencers.clear();
        self.serialized_survivor_branches.clear();

        let mut c = -1;
        for gen in generation_collected {
            if *gen == 1 {
                c += 1;
            }
        }

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

            for branch in self.append_referencers(pinfo, *object_id, 3)
            {
                *self.serialized_survivor_branches.entry(branch).or_insert(0u64) += 1;
            }
        }

        info!("Successfully processed surviving references :)");

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
        // Security timeout
        detach_after_duration::<GCSurvivorsProfiler>(&self, 120);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, GCSurvivorsProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

        if self.serialized_survivor_branches.len() == 0 {
            report.write_line("**Profiler was unable to get a GC surviving references callback! (120 seconds timeout)**".to_string());
        }
        else
        {
            report.write_line(format!("# GC Survivors Report"));
            report.write_line(format!("## Surviving References by Class"));
    
            for surviving_reference in self.serialized_survivor_branches.iter().sorted_by_key(|x| -(*x.1 as i128)) {
                report.write_line(format!("- ({}) {}", surviving_reference.1, surviving_reference.0));
            }
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
        info!("Collecting surviving references...");

        for object_id in object_id_range_start {
            // TODO: Only add zouz that were not gen 2 ?
            *self.surviving_references.entry(*object_id).or_insert(0) += 1;
        }

        Ok(())
    }
}

/* // Does this callback even works?
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
*/

impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}