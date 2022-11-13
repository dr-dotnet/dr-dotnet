// Workflow:
// - Start profiling `SurvivingReferences`
// - On the first full garbage collection, store all surviving references (their ids) in a set. Let's call this set A.
// - On the next full garbage collection
//     - Make a copy of A, let's call it B
//     - Remove each surviving reference from B
//         - If remove fails, it means it's a new object, add it to A and set an "allocated while profiling" flag on it, and store its current size (retained bytes)
//         - Every reference that remains in B is a reference that has been garbage collected, so you can remove it from A
//  - Keep doing the previous step for several garbage collections
//  - On the second to last full garbage collection, start profiling `ObjectReference` also. This will help understand what is the retention path for that leak.
//  - On the last GC
//      - Cleanup object reference graph (just in case)
//      - Remove every non-flagged entry from A. We have our set of persisting references.
//      - When garbage collection ends, for each flagged entry in A, pull the whole retention path and aggregate count by name + the total increase in retained bytes

use std::collections::{HashMap};
use profiling_api::*;
use uuid::Uuid;
use itertools::Itertools;

use crate::report::*;
use crate::profilers::*;

#[derive(Default, Clone)]
struct ReferenceInfo {
    initial_size: usize,
    allocated_while_profiling: bool
}

#[derive(Default, Clone)]
struct GCInfo {
    generation: i8,
    compacting: bool,
    had_survivors_callback: bool,
    is_last_gc: bool,
}

#[derive(Default, Clone)]
pub struct MemoryLeakProfiler {
    profiler_info: Option<ProfilerInfo>,
    session_id: Uuid,
    object_to_referencers: HashMap<ffi::ObjectID, Vec<ffi::ObjectID>>,
    surviving_references: HashMap<ffi::ObjectID, ReferenceInfo>,
    serialized_survivor_branches: HashMap<String, u64>,
    current_gc_info: GCInfo,
    elegible_gcs_count: u32,
    finished: bool
}

impl Profiler for MemoryLeakProfiler {
    fn get_info() -> ProfilerData {
        return ProfilerData {
            profiler_id: Uuid::parse_str("805A308B-061C-47F3-9B30-F785C3186E83").unwrap(),
            name: "Memory Leaks Profiler".to_owned(),
            description: "Finds managed memory leaks.".to_owned(),
            is_released: true,
        }
    }

    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }
}

impl MemoryLeakProfiler
{
    pub fn append_referencers(&self, info: &ProfilerInfo, object_id: ffi::ObjectID, max_depth: i32) -> Vec<String>
    {
        let mut branches = Vec::new();

        self.append_referencers_recursive(info, object_id, &mut String::new(), -max_depth, &mut branches);

        return branches;
    }

    // Recursively drill through referencers.
    fn append_referencers_recursive(&self, info: &ProfilerInfo, object_id: ffi::ObjectID, branch: &mut String, depth: i32, branches: &mut Vec<String>)
    {
        let gen = match info.get_object_generation(object_id) {
            Ok(gen) => gen.generation,
            Err(_) => ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 // Hack to ignore such failure, in case object does not lie in any heap
        };

        let refname = MemoryLeakProfiler::get_object_class_name(info, object_id);
        branch.push_str(refname.as_str());
        //branch.push_str(format!("{} (Gen {:?})", refname, gen.generation).as_str());

        let mut add_branch = || {
            // Only add branches that include ephemeral objects (gen 0 or 1)
            branches.push(branch.clone());
        };

        // Escape in case of circular references (could be done in a better way)
        if depth > 0 {
            add_branch();
            return;
        }

        match self.object_to_referencers.get(&object_id) {
            Some(referencers) => {
                let branch_current_len = branch.len();

                for i in 0..referencers.len() {
                    if i == 0 {
                        // Same branch, we keep on this same branch
                        branch.push_str(" > ");
                        self.append_referencers_recursive(info, referencers[0], branch, depth + 1, branches);
                    }
                    else {
                        // New branch. We clone the current branch to append next holders
                        let mut branch_copy = branch[..branch_current_len].to_string();
                        branch_copy.push_str(" > ");
                        self.append_referencers_recursive(info, referencers[i], &mut branch_copy, depth + 1, branches);
                    }
                }
            },
            None => {
                add_branch();
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
                MemoryLeakProfiler::get_inner_type(info, array_class_info.element_class_id.unwrap(), array_dimension)
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
                let class_id = MemoryLeakProfiler::get_inner_type(info, class_id, &mut array_dimension);
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

impl CorProfilerCallback for MemoryLeakProfiler
{
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), ffi::HRESULT>
    {
        // Only build reference tree on the final GC
        if !self.current_gc_info.is_last_gc {
            return Ok(());
        }

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        info!("Collecting references...");

        for x in object_ref_ids {
            match self.object_to_referencers.get_mut(x) {
                Some(referencers) => referencers.push(object_id),
                None => { self.object_to_referencers.insert(*x, vec![object_id]); },
            };
        }
        
        Ok(())
    }
}

impl CorProfilerCallback2 for MemoryLeakProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT>
    {
        info!("GC started on gen {} for reason {:?}", extensions::get_gc_gen(&generation_collected), reason);

        if self.finished {
            return Ok(());
        }

        let gc_gen = extensions::get_gc_gen(&generation_collected);
        self.current_gc_info.generation = gc_gen;

        if gc_gen < 2 {
            return Ok(());
        }

        self.elegible_gcs_count += 1;

        // Stop after X eligible GCs
        // Todo: Also use time
        self.current_gc_info.is_last_gc = self.elegible_gcs_count > 5;

        // Data from previous garbage collections are no longer valid, so we clear it when a new garbage collection starts.
        self.object_to_referencers.clear();
        self.serialized_survivor_branches.clear();      

        //let pinfo = self.profiler_info();
        //pinfo.force_gc(); // Compacting or not? Which Gen?
        //pinfo.generation

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT>
    {
        info!("GC finished");

        if self.finished {
            return Ok(())
        }

        if !self.current_gc_info.is_last_gc {
            return Ok(());
        }

        // Profiler may not be detached yet after this callback, still we want to stop tracking references
        self.finished = true;

        // Disable profiling to free some resources
        match self.profiler_info().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:x}", hresult)
        }

        // Request detach
        let profiler_info = self.profiler_info().clone();
        profiler_info.request_profiler_detach(3000).ok();

        // Post-process tracked persisting references
        for (object_id, ref_info) in self.surviving_references.iter() {
            if !ref_info.allocated_while_profiling {
                continue;
            }
            info!("Persisting object id: {}", *object_id);

            let info = self.profiler_info();
            for branch in self.append_referencers(info, *object_id, 6) {
                *self.serialized_survivor_branches.entry(branch).or_insert(0u64) += 1;
            }
        }

        info!("Successfully processed persisting objects");

        // We can free some memory
        self.object_to_referencers.clear();
        self.surviving_references.clear();

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
        // Security timeout
        detach_after_duration::<MemoryLeakProfiler>(&self, 320, None);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT>
    {
        let session = Session::get_session(self.session_id, MemoryLeakProfiler::get_info());

        let mut report = session.create_report("summary.md".to_owned());

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

impl CorProfilerCallback4 for MemoryLeakProfiler
{
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_id_range_start: &[ffi::ObjectID], c_object_id_range_length: &[usize]) -> Result<(), ffi::HRESULT>
    {
        info!("Surviving references 2 ({} objects survived)", object_id_range_start.len());

        if self.finished {
            return Ok(());
        }

        // SurvivingReferences2 happens on non-compacting garbage collections
        self.current_gc_info.compacting = false;
        self.current_gc_info.had_survivors_callback = true;

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        if self.surviving_references.len() == 0
        {
            // First time collecting surviving references
            for i in 0..object_id_range_start.len() {
                let id = object_id_range_start[i];
                if id != 0 {
                    self.surviving_references.insert(id, ReferenceInfo { initial_size: c_object_id_range_length[i], allocated_while_profiling: false });
                }
            }
        }
        else
        {
            // Create a copy to know which objects are new and which ones are dead
            let mut surviving_references_copy = self.surviving_references.clone();

            for i in 0..object_id_range_start.len() {
                let id = object_id_range_start[i];
                if id != 0 {
                    match surviving_references_copy.remove(&id) {
                        Some(removed_entry) => {
                            // Todo: record number of GCs it survived ?
                        },
                        None => {
                            // Object was not present, so we add it as "allocated while profiling"
                            self.surviving_references.insert(id, ReferenceInfo { initial_size: c_object_id_range_length[i], allocated_while_profiling: true });
                        },
                    }
                    self.surviving_references.insert(id, ReferenceInfo { initial_size: c_object_id_range_length[i], allocated_while_profiling: true });
                }
            }

            // Everything that remains in our copy is a dead reference, so we remove it from original map
            for (id, reference_info) in surviving_references_copy {
                match self.surviving_references.remove(&id) {
                    Some(removed_entry) => (),
                    None => error!("Should not happen! [surviving_references_2]")
                }
            }
        }

        // Todo: Return HRESULT ?
        Ok(())
    }

    // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-movedreferences2-method
    fn moved_references_2(&mut self, old_object_id_range_start: &[ffi::ObjectID], new_object_id_range_start: &[ffi::ObjectID], object_id_range_length: &[usize]) -> Result<(), ffi::HRESULT>
    {
        info!("Moved references 2 ({} objects moved)", old_object_id_range_start.len());

        if self.finished {
            return Ok(());
        }

        // MovedReferences happens on compacting garbage collections
        self.current_gc_info.compacting = true;
        self.current_gc_info.had_survivors_callback = true;

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        if self.surviving_references.len() == 0
        {
            // First time collecting surviving+moved references
            for i in 0..new_object_id_range_start.len() {
                let id = new_object_id_range_start[i];
                if id != 0 {
                    self.surviving_references.insert(id, ReferenceInfo { initial_size: new_object_id_range_start[i], allocated_while_profiling: false });
                }
            }
        }
        else
        {
            // Create a copy to know which objects are new and which ones are dead
            let mut surviving_references_copy = self.surviving_references.clone();

            for i in 0..old_object_id_range_start.len() {
                let id = old_object_id_range_start[i];
                if id != 0 {
                    match surviving_references_copy.remove(&id) {
                        Some(removed_entry) => {
                            // Todo: record number of GCs it survived ?
                        },
                        None => {
                            // Object was not present, so we add it (its new id, not the old one) as "allocated while profiling".
                            self.surviving_references.insert(id, ReferenceInfo { initial_size: new_object_id_range_start[i], allocated_while_profiling: true });
                        },
                    }
                    self.surviving_references.insert(id, ReferenceInfo { initial_size: new_object_id_range_start[i], allocated_while_profiling: true });
                }
            }

            // Everything that remains in our copy is a dead reference, so we remove it from original map
            for (id, reference_info) in surviving_references_copy {
                match self.surviving_references.remove(&id) {
                    Some(removed_entry) => (),
                    None => error!("Should not happen! [surviving_references_2]")
                }
            }
        }

        // Todo: Return HRESULT ? "Profilers can return an HRESULT that indicates failure from the MovedReferences2 method, to avoid calling the second method"
        Ok(())
    }
}

impl CorProfilerCallback5 for MemoryLeakProfiler {}
impl CorProfilerCallback6 for MemoryLeakProfiler {}
impl CorProfilerCallback7 for MemoryLeakProfiler {}
impl CorProfilerCallback8 for MemoryLeakProfiler {}
impl CorProfilerCallback9 for MemoryLeakProfiler {}