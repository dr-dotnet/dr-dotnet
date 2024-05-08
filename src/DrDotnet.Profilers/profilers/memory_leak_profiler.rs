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

use itertools::Itertools;
use std::collections::HashMap;

use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::utils::NameResolver;

#[derive(Default, Clone)]
struct ReferenceInfo {
    //initial_size: usize,
    first_gc_survived: u16,
    last_gc_survived: u16,
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
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_referencers: HashMap<ffi::ObjectID, Vec<ffi::ObjectID>>,
    surviving_references: HashMap<ffi::ObjectID, ReferenceInfo>,
    serialized_survivor_branches: HashMap<String, u64>,
    current_gc_info: GCInfo,
    elegible_gcs_count: u16,
    finished: bool,
}

impl Profiler for MemoryLeakProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A308B-061C-47F3-9B30-F785C3186E83".to_owned(),
            name: "Memory Leaks Profiler".to_owned(),
            description: "Finds managed memory leaks.".to_owned(),
            is_released: false,
            ..std::default::Default::default()
        };
    }
}

impl MemoryLeakProfiler {
    pub fn append_referencers(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, max_depth: i32) -> Vec<String> {
        let mut branches = Vec::new();

        self.append_referencers_recursive(info, object_id, &mut String::new(), -max_depth, &mut branches);

        return branches;
    }

    // Recursively drill through referencers.
    fn append_referencers_recursive(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, branch: &mut String, depth: i32, branches: &mut Vec<String>) {
        let gen = match info.get_object_generation(object_id) {
            Ok(gen) => gen.generation,
            Err(_) => ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2, // Hack to ignore such failure, in case object does not lie in any heap
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
                        branch.push_str(" < ");
                        self.append_referencers_recursive(info, referencers[0], branch, depth + 1, branches);
                    } else {
                        // New branch. We clone the current branch to append next holders
                        let mut branch_copy = branch[..branch_current_len].to_string();
                        branch_copy.push_str(" < ");
                        self.append_referencers_recursive(info, referencers[i], &mut branch_copy, depth + 1, branches);
                    }
                }
            }
            None => {
                add_branch();
            }
        }
    }

    fn get_inner_type(info: &ClrProfilerInfo, class_id: usize, array_dimension: &mut usize) -> usize {
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
        match info.is_array_class(class_id) {
            Ok(array_class_info) => {
                *array_dimension = *array_dimension + 1;
                // TODO: Handle array_class_info.rank
                MemoryLeakProfiler::get_inner_type(info, array_class_info.element_class_id.unwrap(), array_dimension)
            }
            Err(_) => class_id,
        }
    }

    // Todo: Share code?
    fn get_object_class_name(clr: &ClrProfilerInfo, object_id: ffi::ObjectID) -> String {
        let mut array_dimension = 0;

        let mut name = match clr.get_class_from_object(object_id) {
            Ok(class_id) => {
                // As the class could be an array, we recursively dig until we find the inner type that is not an array
                let class_id = MemoryLeakProfiler::get_inner_type(clr, class_id, &mut array_dimension);
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
                // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
                clr.clone().get_class_name(class_id)
            }
            Err(error) => format!("error: {:?}", error),
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

impl CorProfilerCallback for MemoryLeakProfiler {
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), ffi::HRESULT> {
        // Only build reference tree on the final GC
        if !self.current_gc_info.is_last_gc {
            return Ok(());
        }

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        //info!("Collecting references...");

        // Create dependency tree, but from object to referencers, instead of object to its references.
        // This is usefull for being able to browse from any object back to its roots.
        for object_ref_id in object_ref_ids {
            match self.object_to_referencers.get_mut(object_ref_id) {
                Some(referencers) => referencers.push(object_id),
                None => {
                    self.object_to_referencers.insert(*object_ref_id, vec![object_id]);
                }
            };
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for MemoryLeakProfiler {
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT> {
        info!(
            "GC started on gen {} for reason {:?}",
            ClrProfilerInfo::get_gc_gen(&generation_collected),
            reason
        );

        if self.finished {
            return Ok(());
        }

        let gc_gen = ClrProfilerInfo::get_gc_gen(&generation_collected);
        self.current_gc_info.generation = gc_gen;

        if gc_gen < 2 {
            return Ok(());
        }

        self.elegible_gcs_count += 1;

        // Stop after X eligible GCs
        // Todo: Also use time
        self.current_gc_info.is_last_gc = self.elegible_gcs_count > 5;

        // Data from previous garbage collections are no longer valid, so we clear it when a new garbage collection starts.
        //self.object_to_referencers.clear();
        //self.serialized_survivor_branches.clear();

        //let pinfo = self.profiler_info();
        //pinfo.force_gc(); // Compacting or not? Which Gen?
        //pinfo.generation

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT> {
        info!("GC finished");

        if self.finished {
            return Ok(());
        }

        if self.current_gc_info.had_survivors_callback {
            let mut dead_refs = Vec::new();
            for (id, ref_info) in self.surviving_references.iter() {
                if ref_info.last_gc_survived != self.elegible_gcs_count {
                    dead_refs.push(*id);
                }
            }
            info!("Untracking {} dead refs", dead_refs.len());
            for dead_ref in dead_refs {
                self.surviving_references.remove(&dead_ref);
            }
        }

        if !self.current_gc_info.is_last_gc {
            return Ok(());
        }

        // Profiler may not be detached yet after this callback, still we want to stop tracking references
        self.finished = true;

        // Disable profiling to free some resources
        match self.clr().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:?}", hresult),
        }

        // Post-process tracked persisting references
        info!("Building dependency tree... {} objects to process", self.surviving_references.len());
        for (object_id, ref_info) in self.surviving_references.iter() {
            // Was already there when profiling started, so not interesting for us
            if !ref_info.first_gc_survived == 1 {
                continue;
            }

            // Todo: Somehow estimate the "leaky profile" from first and last gc
            // One option is to store for each branch a Map<gc_number, count> and do curve fitting on this
            // Aso it could be interesting to track size increase difference

            //info!("Persisting object id: {}", *object_id);

            let info = self.clr();
            let gen = info.get_object_generation(*object_id).unwrap();
            if gen.generation != ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                error!("Object is supposed to be in gen 2 but is {:?}", gen.generation);
            }

            let info = self.clr();
            for branch in self.append_referencers(info, *object_id, 6) {
                *self.serialized_survivor_branches.entry(branch).or_insert(0u64) += 1;
            }
        }

        info!("Successfully processed persisting objects");

        // Request detach
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();

        // We can free some memory
        self.object_to_referencers.clear();
        self.surviving_references.clear();

        Ok(())
    }
}

impl CorProfilerCallback3 for MemoryLeakProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT> {
        // Security timeout
        detach_after_duration::<MemoryLeakProfiler>(&self, 320);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        let mut report = self.session_info.create_report("summary.md".to_owned());

        if self.serialized_survivor_branches.len() == 0 {
            report.write_line("**Profiler was unable to get a GC surviving references callback! (120 seconds timeout)**".to_string());
        } else {
            report.write_line(format!("# GC Survivors Report"));
            report.write_line(format!("## Surviving References by Class"));

            for surviving_reference in self.serialized_survivor_branches.iter().sorted_by_key(|x| -(*x.1 as i128)) {
                report.write_line(format!("- ({}) {}", surviving_reference.1, surviving_reference.0));
            }
        }

        self.session_info.finish();

        info!("Report written");

        Ok(())
    }
}

impl CorProfilerCallback4 for MemoryLeakProfiler {
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_ids: &[ffi::ObjectID], object_lengths: &[usize]) -> Result<(), ffi::HRESULT> {
        info!("Surviving references 2 ({} objects survived)", object_ids.len());

        if self.finished {
            return Ok(());
        }

        // SurvivingReferences2 happens on non-compacting garbage collections
        self.current_gc_info.compacting = false; // bullshit ?
        self.current_gc_info.had_survivors_callback = true;

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        for i in 0..object_ids.len() {
            let id = object_ids[i];
            if id == 0 {
                continue;
            }
            let info = self.clr();
            let gen = info.get_object_generation(id).unwrap();
            if gen.generation != ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                continue;
            }

            match self.surviving_references.get_mut(&id) {
                // Reference was already tracked: we update the last gc info
                Some(ref_info) => ref_info.last_gc_survived = self.elegible_gcs_count,
                // Reference wasn't tracked: we start tracking it
                None => {
                    self.surviving_references.insert(
                        id,
                        ReferenceInfo {
                            //initial_size: object_lengths[i],
                            first_gc_survived: self.elegible_gcs_count,
                            last_gc_survived: self.elegible_gcs_count,
                        },
                    );
                }
            };
        }

        // Return HRESULT because "Profilers can return an HRESULT that indicates failure from the MovedReferences2 method, to avoid calling the second method"
        Err(ffi::HRESULT::E_FAIL)
    }

    // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-movedreferences2-method
    fn moved_references_2(&mut self, old_object_ids: &[ffi::ObjectID], new_object_ids: &[ffi::ObjectID], object_lengths: &[usize]) -> Result<(), ffi::HRESULT> {
        info!("Moved references 2 ({} objects moved)", old_object_ids.len());

        if self.finished {
            return Ok(());
        }

        // MovedReferences happens on compacting garbage collections
        self.current_gc_info.compacting = true; // bullshit ?
        self.current_gc_info.had_survivors_callback = true;

        // Only process gen 2
        if self.current_gc_info.generation < 2 {
            return Ok(());
        }

        let new_tracked_refs = 0;
        let mut moved_tracked_refs = 0;

        for i in 0..old_object_ids.len() {
            let old_id = old_object_ids[i];
            let new_id = new_object_ids[i];
            if old_id == 0 {
                continue;
            }
            let info = self.clr();
            let gen = info.get_object_generation(old_id).unwrap();
            if gen.generation != ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                continue;
            }

            match self.surviving_references.remove(&old_id) {
                // Reference was already tracked: we update its ID and the last gc info
                Some(ref_info) => {
                    let mut ref_info = ref_info.clone();
                    ref_info.last_gc_survived = self.elegible_gcs_count;
                    self.surviving_references.insert(new_id, ref_info);
                    moved_tracked_refs += 1;
                }
                // Reference wasn't tracked: we start tracking it
                None => {
                    self.surviving_references.insert(
                        new_id,
                        ReferenceInfo {
                            //initial_size: object_lengths[i],
                            first_gc_survived: self.elegible_gcs_count,
                            last_gc_survived: self.elegible_gcs_count,
                        },
                    );
                }
            };
        }

        info!("Tracking {} new refs", new_tracked_refs);
        info!("Updated {} moved refs", moved_tracked_refs);

        // Return HRESULT because "Profilers can return an HRESULT that indicates failure from the MovedReferences2 method, to avoid calling the second method"
        Err(ffi::HRESULT::E_FAIL)
    }
}

impl CorProfilerCallback5 for MemoryLeakProfiler {}
impl CorProfilerCallback6 for MemoryLeakProfiler {}
impl CorProfilerCallback7 for MemoryLeakProfiler {}
impl CorProfilerCallback8 for MemoryLeakProfiler {}
impl CorProfilerCallback9 for MemoryLeakProfiler {}
