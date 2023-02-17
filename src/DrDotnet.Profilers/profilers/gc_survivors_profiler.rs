// Workflow:
// - On GC start, clear everything
// - Collect all GC roots
// - On GC end, for each GC root collected, iterate over objects it references (recursively). That gives use a tree of survivors gor the last GC
// - Stop profiling and make a report

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::api::*;
use crate::macros::*;
use crate::profilers::*;

#[derive(Default, Clone)]
pub struct GCSurvivorsProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_references: HashMap<ffi::ObjectID, Vec<ffi::ObjectID>>,
    serialized_survivor_branches: HashMap<String, u64>,
    root_references: HashSet<ffi::ObjectID>,
    // todo: add state
}

impl Profiler for GCSurvivorsProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-F785C3186E86".to_owned(),
            name: "GC Survivors".to_owned(),
            description: "After a garbage collection, iterate over GC roots and browse through references recursively until an ephemeral object is found (gen 0 or 1). \
                Then, list such retained objects with the chain of references, along with the count of such occurence. \
                Timeouts after 320s if no garbage collection happened or if did not succeed to catch any callback.".to_owned(),
            is_released: true,
            ..std::default::Default::default()
        }
    }
}

impl GCSurvivorsProfiler
{
    pub fn append_references(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, max_depth: i32) -> Vec<String>
    {
        let mut branches = Vec::new();

        self.append_references_recursive(info, object_id, &mut String::new(), -max_depth, &mut branches);

        return branches;
    }

    // Recursively drill through references until we find a gen 2 object.
    fn append_references_recursive(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, branch: &mut String, depth: i32, branches: &mut Vec<String>)
    {
        let gen = match info.get_object_generation(object_id) {
            Ok(gen) => gen.generation,
            Err(_) => ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 // Hack to ignore such failure, in case object does not lie in any heap
        };

        let refname = GCSurvivorsProfiler::get_object_class_name(info, object_id);
        branch.push_str(refname.as_str());
        //branch.push_str(format!("{} (Gen {:?})", refname, gen.generation).as_str());

        let mut add_branch = || {
            // Only add branches that include ephemeral objects (gen 0 or 1)
            if gen == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_0 || gen == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_1 {
                branches.push(branch.clone());
            }
        };

        // Escape in case of circular references (could be done in a better way)
        if depth > 0 {
            add_branch();
            return;
        }

        match self.object_to_references.get(&object_id) {
            Some(referencers) => {
                let branch_current_len = branch.len();

                for i in 0..referencers.len() {
                    if i == 0 {
                        // Same branch, we keep on this same branch
                        branch.push_str(" > ");
                        self.append_references_recursive(info, referencers[0], branch, depth + 1, branches);
                    }
                    else {
                        // New branch. We clone the current branch to append next holders
                        let mut branch_copy = branch[..branch_current_len].to_string();
                        branch_copy.push_str(" > ");
                        self.append_references_recursive(info, referencers[i], &mut branch_copy, depth + 1, branches);
                    }
                }
            },
            None => {
                add_branch();
            }
        }
    }

    fn get_inner_type(info: &ClrProfilerInfo, class_id: usize, array_dimension: &mut usize) -> usize
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

    // Todo: Share code ?
    fn get_object_class_name(clr: &ClrProfilerInfo, object_id: ffi::ObjectID) -> String
    {
        let mut array_dimension = 0;

        let mut name = match clr.get_class_from_object(object_id) {
            Ok(class_id) => {
                // As the class could be an array, we recursively dig until we find the inner type that is not an array
                let class_id = GCSurvivorsProfiler::get_inner_type(clr, class_id, &mut array_dimension);
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
                match clr.get_class_id_info(class_id) {
                    Ok(class_info) => clr.get_type_name(class_info.module_id, class_info.token),
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

impl CorProfilerCallback1 for GCSurvivorsProfiler
{
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), ffi::HRESULT>
    {
        if self.root_references.len() == 0 {
            return Ok(())
        }

        info!("Collecting references...");

        self.object_to_references.insert(object_id, object_ref_ids.to_vec());

        Ok(())
    }
}

impl CorProfilerCallback2 for GCSurvivorsProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        // Data from previous garbage collections are no longer valid, so we clear it when a new garbage collection starts.
        self.object_to_references.clear();
        self.serialized_survivor_branches.clear();
        self.root_references.clear();

        info!("GC started on gen {} for reason {:?}", ClrProfilerInfo::get_gc_gen(&generation_collected), reason);

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT>
    {
        info!("GC finished");

        if self.root_references.len() == 0 {
            return Ok(())
        }

        // Disable profiling to free some resources
        match self.clr().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        info!("Roots to process: {}", self.root_references.len());

        for object_id in self.root_references.iter() {
            let info = self.clr();
            info!("Root id: {}", *object_id);
            for branch in self.append_references(info, *object_id, 6) {
                *self.serialized_survivor_branches.entry(branch).or_insert(0u64) += 1;
            }
        }

        info!("Successfully processed surviving roots :)");

        // We're done, we can detach :)
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();

        Ok(())
    }

    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback2-rootreferences2-method
    fn root_references_2(&mut self, root_ref_ids: &[ffi::ObjectID], root_kinds: &[ffi::COR_PRF_GC_ROOT_KIND], root_flags: &[ffi::COR_PRF_GC_ROOT_FLAGS], root_ids: &[ffi::UINT_PTR]) -> Result<(), ffi::HRESULT>
    {
        // Only track in case of gen 2!

        info!("Root references ({})", root_ids.len());

        for i in 0..root_ref_ids.len() {
            let id = root_ref_ids[i];
            //info!("Root '{}' of kind: {:?}", GCSurvivorsProfiler::get_object_class_name(self.profiler_info(), id), root_kinds[i]);
            if id != 0 {
                // For some reasons, this callback may return several times the same object references, so we use a duplication free collection
                self.root_references.insert(id);
            }
        }

        Ok(())
    }
}

impl CorProfilerCallback3 for GCSurvivorsProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT>
    {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        // Security timeout
        detach_after_duration::<GCSurvivorsProfiler>(&self, 320, None);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let mut report = self.session_info.create_report("summary.md".to_owned());

        if self.serialized_survivor_branches.len() == 0 {
            report.write_line("**Profiler was unable to get a GC surviving references callback! (120 seconds timeout)**".to_string());
        }
        else {
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

impl CorProfilerCallback4 for GCSurvivorsProfiler {}
impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}