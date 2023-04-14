// Workflow:
// - On GC start, clear everything
// - Collect all GC roots
// - On GC end, for each GC root collected, iterate over objects it references (recursively). That gives use a tree of survivors gor the last GC
// - Stop profiling and make a report

use std::cmp::min;
use dashmap::{DashMap, DashSet};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Keys;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use itertools::Itertools;
use std::thread;
use thousands::{digits, Separable, SeparatorPolicy};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::ffi::ClassID;
use crate::ffi::ObjectID;
use crate::api::*;
use crate::api::ffi::{COR_PRF_GC_GENERATION_RANGE, HRESULT};
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::TreeNode;


#[derive(Default)]
pub struct GCSurvivorsProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_referencers: HashMap<ObjectID, Vec<ObjectID>>,
    is_triggered_gc: AtomicBool,
    surviving_references: DashMap<ObjectID, usize>,
    sequences: HashMap<Vec<ClassID>, References>,
}

#[derive(Clone, Default, Debug)]
pub struct References(HashMap<ObjectID, usize>);

// Implement AddAssign for get_inclusive_value to be usable
impl AddAssign<&References> for References {
    fn add_assign(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }
}

impl Display for References {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let policy = SeparatorPolicy {
            separator: ",",
            groups:    &[3],
            digits:    digits::ASCII_DECIMAL,
        };
        
        let nb_objects = self.0.len();
        let total_size: usize = self.0.values().sum();

        let total_size_str = if total_size > 0 { total_size.separate_by_policy(policy) } else { "???".to_string() };

        write!(f, "({total_size_str} bytes) - {nb_objects}")
    }
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

// - trigger GC
// on gc
// - get surviving references from gen 2 (HashSet<ObjectID> or HashMap<ObjectID, usize (bytes)>)
// - build map from object to referencers (inverted)
// after gc
// - for each surviving reference
//   - get all retention paths recursively (they all start with the surviving ref ID)
//   - each path is a vec or class IDs. With that get objectid size

impl GCSurvivorsProfiler
{
    pub fn append_references(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, max_depth: i32) -> Vec<Vec<ClassID>>
    {
        let mut branches: Vec<Vec<ClassID>> = Vec::new();

        self.append_references_recursive(info, object_id, &mut Vec::new(), -max_depth, &mut branches);

        return branches;
    }
    
    fn append_references_recursive(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, branch: &mut Vec<ClassID>, depth: i32, branches: &mut Vec<Vec<ClassID>>)
    {
        let class_id = info.get_class_from_object(object_id);
        if class_id.is_err() {
            return; // do something else?
        }
        branch.push(class_id.unwrap());

        let mut add_branch = || {
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
                        self.append_references_recursive(info, referencers[0], branch, depth + 1, branches);
                    }
                    else {
                        // New branch. We clone the current branch to append next holders
                        let mut branch_copy = branch[..branch_current_len].to_vec();
                        self.append_references_recursive(info, referencers[i], &mut branch_copy, depth + 1, branches);
                    }
                }
            },
            None => {
                add_branch();
            }
        }
    }

    fn print_html(&self, tree: &TreeNode<ClassID, References>, is_same_level: bool, report: &mut Report)
    {
        let refs = &tree.get_inclusive_value();
        let nb_objects = refs.0.len();
        let class_name = self.get_class_name(tree.key);

        if !is_same_level {
            report.write(format!("\n<details><summary><span>{refs}</span>{class_name}</summary>"));
        } else {
            report.write(format!("\n<li><span>{refs}</span>{class_name}</li>"));
        }

        for child in &tree.children {
            let has_same_alignment = (child.children.is_empty() || child.children.len() == 1)
                && nb_objects == child.get_inclusive_value().0.len();
            
            if has_same_alignment && !is_same_level {
                report.write(format!("\n<ul>\n"));
            }
            self.print_html(child, has_same_alignment, report);

            if has_same_alignment && !is_same_level {
                report.write(format!("\n</ul>\n"));
            }
        }

        if !is_same_level {
            report.write(format!("\n</details>\n"));
        }
    }

    fn get_inner_type(&self, class_id: usize, array_dimension: &mut usize) -> usize
    {
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-isarrayclass-method
        match self.clr().is_array_class(class_id) {
            Ok(array_class_info) => {
                *array_dimension = *array_dimension + 1;
                // TODO: Handle array_class_info.rank
                self.get_inner_type(array_class_info.element_class_id.unwrap(), array_dimension)
            },
            Err(_) => class_id,
        }
    }
    
    fn get_class_name(&self, class_id: ffi::ClassID) -> String
    {
        let mut array_dimension = 0;

        let class_id = self.get_inner_type(class_id, &mut array_dimension);
        
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getclassidinfo-method
        // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo2-getclassidinfo2-method
        let mut name = match self.clr().get_class_id_info(class_id) {
            Ok(class_info) => self.clr().get_type_name(class_info.module_id, class_info.token),
            _ => "unknown2".to_owned()
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

    fn build_tree_and_write_report(&mut self) -> Result<(), ffi::HRESULT>
    {
        info!("Building report");
        
        let mut tree = TreeNode::build_from_sequences(&self.sequences, 0);

        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len()));
        
        // fn print<T, V, F>(tree: &TreeNode<T, V>, depth: usize, format: &F)
        //     where F: Fn(&TreeNode<T, V>) -> String
        // {
        //     let tabs = " ".repeat(depth);
        //     info!("{}- {}", tabs, format(tree));
        // 
        //     for child in &tree.children {
        //         print(child, depth + 1, format);
        //     }
        // }
        // print(&tree, 0, &|node: &TreeNode<ClassID, References>| format!("{} [inc:{}, exc:{:?}]",  self.get_class_name(node.key), node.get_inclusive_value(), node.value));

        let nb_classes = tree.children.len();
        let nb_objects: usize = tree.children.iter().map(|x| x.get_inclusive_value().0.len()).sum();

        let mut report = self.session_info.create_report("summary.html".to_owned());

        report.write_line(format!("<h2>GC Survivors Report</h2>"));
        report.write_line(format!("<h3>{nb_objects} surviving objects of {nb_classes} classes..</h3>"));

        for tree_node in tree.children {
            self.print_html(&tree_node, false, &mut report);
        }

        info!("Report written");

        Ok(())
    }
    
}

impl CorProfilerCallback for GCSurvivorsProfiler
{
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), ffi::HRESULT>
    {
        if !self.is_triggered_gc.load(Ordering::Relaxed) {
            error!("Early return of object_references because GC wasn't forced yet");
            // Early return if we received an event before the forced GC started
            return Ok(());
        }

        // Create dependency tree, but from object to referencers, instead of object to its references.
        // This is usefull for being able to browse from any object back to its roots.
        for object_ref_id in object_ref_ids {
            self.object_to_referencers.entry(*object_ref_id)
                .and_modify(|referencers| referencers.push(object_id))
                .or_insert(vec![object_id]);
        }

        Ok(())
    }
}

impl CorProfilerCallback2 for GCSurvivorsProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), ffi::HRESULT> {
        info!("garbage_collection_started on gen {} for reason {:?}", ClrProfilerInfo::get_gc_gen(&generation_collected), reason);
        
        if reason == ffi::COR_PRF_GC_REASON::COR_PRF_GC_INDUCED {
            self.is_triggered_gc.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), ffi::HRESULT>
    {
        info!("garbage_collection_finished");
        
        if !self.is_triggered_gc.load(Ordering::Relaxed) {
            error!("Early return of garbage_collection_finished because GC wasn't forced yet");
            // Early return if we received an event before the forced GC started
            return Ok(());
        }

        // Disable profiling to free some resources
        match self.clr().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:?}", hresult)
        }

        // Post-process tracked persisting references
        info!("Building graph of surviving references... {} objects to process", self.surviving_references.len());
        for surviving_reference in self.surviving_references.iter() {
            let info = self.clr();
            let object_id = surviving_reference.key().clone();
            let count = surviving_reference.value().clone();
            match info.get_object_generation(object_id) {
                Ok(gen) => {
                    // debug!("Surviving object id ({object_id}) generation: {:?}", gen.generation);
                    // we care only for objects from GEN 2
                    if gen.generation != ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2 {
                        continue;
                    }
                }
                Err(e) => { debug!("Error ({:?}) getting generation of object id: {object_id}", e); continue; }
            }
            for branch in self.append_references(info, object_id, 10) {
                
                self.sequences.entry(branch)
                    .and_modify(|referencers| {referencers.0.insert(object_id.clone(), count);})
                    .or_insert(References(HashMap::from([(object_id.clone(), count)])));
            }
        }

        let _ = self.build_tree_and_write_report();
        
        // We're done, we can detach :)
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();

        Ok(())
    }
}

impl CorProfilerCallback3 for GCSurvivorsProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), ffi::HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), ffi::HRESULT>
    {
        // The ForceGC method must be called only from a thread that does not have any profiler callbacks on its stack. 
        // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-forcegc-method
        let p_clone = self.clr().clone();

        let _ = thread::spawn(move || {
            debug!("Force GC");
            
            match p_clone.force_gc() {
                Ok(_) => debug!("GC Forced!"),
                Err(hresult) => error!("Error forcing GC: {:?}", hresult)
            };
        }).join();
        
        // Security timeout
        detach_after_duration::<GCSurvivorsProfiler>(&self, 360, None);

        Ok(())
    }
}

impl CorProfilerCallback4 for GCSurvivorsProfiler
{
    // https://docs.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-survivingreferences2-method
    fn surviving_references_2(&mut self, object_ids: &[ffi::ObjectID], object_lengths: &[usize]) -> Result<(), ffi::HRESULT>
    {
        info!("surviving_references_2 ({} objects survived) on thread {:?}", object_ids.len(), std::thread::current().id());
        
        if !self.is_triggered_gc.load(Ordering::Relaxed) {
            error!("Early return of surviving_references_2 because GC wasn't forced yet");
            return Ok(());
        }

        for i in 0..object_ids.len() {
            let id = object_ids[i];
            if id == 0 {
                continue; // skip native frame
            }
            //debug!("surviving_references_2 obj id: {id}");

            // we care only for objects from GEN 2 but cannot check it here while GC is not finished
            let size = object_lengths[i];
            self.surviving_references.entry(id)
                .and_modify(|mut s| *s = size)
                .or_insert(size);
        }

        // Return HRESULT because "Profilers can return an HRESULT that indicates failure from the MovedReferences2 method, to avoid calling the second method"
        Err(ffi::HRESULT::E_FAIL)
    }

    // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilercallback4-movedreferences2-method
    fn moved_references_2(&mut self, old_object_ids: &[ffi::ObjectID], new_object_ids: &[ffi::ObjectID], object_lengths: &[usize]) -> Result<(), ffi::HRESULT>
    {
        info!("moved_references_2 ({} objects moved) on thread {:?}", old_object_ids.len(), std::thread::current().id());

        if !self.is_triggered_gc.load(Ordering::Relaxed) {
            error!("Early return of moved_references_2 because GC wasn't forced yet");
            return Ok(());
        }

        let mut new_tracked_refs = 0;
        let mut moved_tracked_refs = 0;

        for i in 0..old_object_ids.len() {
            let old_id = old_object_ids[i];
            let new_id = new_object_ids[i];
            let new_size = object_lengths[i];
            
            if old_id == 0 {
                continue;
            }

            if self.surviving_references.remove(&old_id).is_some() {
                moved_tracked_refs += 1;
            }

            self.surviving_references.entry(new_id)
                .and_modify(|mut s| *s = new_size)
                .or_insert(new_size);
        }

        info!("Tracking {} new refs",  old_object_ids.len() - moved_tracked_refs);
        info!("Updated {} moved refs", moved_tracked_refs);

        // Return HRESULT because "Profilers can return an HRESULT that indicates failure from the MovedReferences2 method, to avoid calling the second method"
        Err(ffi::HRESULT::E_FAIL)
    }
}

impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}