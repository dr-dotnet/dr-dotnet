// Workflow:
// - On GC start, clear everything
// - Collect all GC roots
// - On GC end, for each GC root collected, iterate over objects it references (recursively). That gives use a tree of survivors gor the last GC
// - Stop profiling and make a report

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use dashmap::DashMap;
use thousands::{digits, Separable, SeparatorPolicy};

use crate::ffi::*;
use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::TreeNode;

#[derive(Default)]
pub struct GCSurvivorsProfiler {
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_referencers: DashMap<ObjectID, Vec<ObjectID>>,
    is_triggered_gc: AtomicBool
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
    pub fn append_references(&self, info: &ClrProfilerInfo, object_id: ffi::ObjectID, max_depth: usize) -> Vec<Vec<ClassID>>
    {
        let mut branches: Vec<Vec<ClassID>> = Vec::new();

        let mut branch: Vec<ClassID> = vec![]; // A branch we update live and copy every time we reached the end of a path
        let mut stack: Vec<(ObjectID, usize)> = vec![(object_id, 0)]; // A stack to iterate without recursions

        while !stack.is_empty() {

            let current_id: (ObjectID, usize) = stack.pop().unwrap();

            // Trim branch until it has the proper depth
            while branch.len() > current_id.1 {
                branch.pop();
            }

            match info.get_class_from_object(current_id.0) {
                Ok(class_id) => branch.push(class_id),
                Err(_) => {
                    error!("Impossible to get class ID for object ID {}", current_id.0);
                    continue;
                }
            }

            match self.object_to_referencers.get(&current_id.0) {
                Some(referencers) => {
                    if referencers.len() == 0 {
                        // We reached the end of a path, copy the branch and add it to our branches
                        branches.push(branch.clone());
                    } else {
                        // Only push new referencers if we are within our allowed depth
                        if current_id.1 < max_depth {
                            for i in 0..referencers.len() {
                                stack.push((referencers[i], current_id.1 + 1));
                            }
                        }
                    }
                },
                None => {
                    // We reached the end of a path, copy the branch and add it to our branches
                    branches.push(branch.clone());
                }
            }
        }

        // Example:
        // M
        // ├─ A
        // │  └─ D
        // ├─ B
        // └─ C
        //    ├─ F
        //    └─ G
        // 0  1  2
        // Iteration 1: We pop M, which is added to the branch. A, B and C are pushed to the stack
        // Iteration 2: We pop C, which is added to the branch. F and G are pushed to the stack
        // Iteration 3: We pop G, which is added to the branch. There are no child, so the branch MCG is completed
        // Iteration 4: We pop F. F depth is 2, but the branch is currently of len 3, so G is removed from the branch. F is then added to the branch. There are no child, so the branch MCF is completed
        // Iteration 5: We pop B. B depth is 1, but the branch is currently of len 3, so C and F are removed from the branch. B is then added to the branch. There are no child, so the branch MB is completed
        // Iteration 6: We pop A. A depth is 1, but the branch is currently of len 2, so B is removed from the branch. A is then added to the branch
        // Iteration 7: We pop D. which is added to the branch. There are no child, so the branch MAD is completed
        // The stack is now empty, we're done :)

        return branches;
    }

    fn print_html(&self, tree: &TreeNode<ClassID, References>, is_same_level: bool, report: &mut Report)
    {
        let refs = &tree.get_inclusive_value();
        let nb_objects = refs.0.len();
        let class_name = self.clr().get_class_name(tree.key);

        if !is_same_level {
            report.write(format!("\n<details><summary><span>{refs}</span>{class_name}</summary>"));
        } else {
            report.write(format!("\n<li><span>{refs}</span>{class_name}</li>"));
        }

        let mut i = 0;
        for child in &tree.children {
            
            // Set a limit to the output
            if i > 100 {
                break;
            }
            
            let has_same_alignment = (child.children.is_empty() || child.children.len() == 1)
                && nb_objects == child.get_inclusive_value().0.len();
            
            if has_same_alignment && !is_same_level {
                report.write(format!("\n<ul>\n"));
            }
            self.print_html(child, has_same_alignment, report);

            if has_same_alignment && !is_same_level {
                report.write(format!("\n</ul>\n"));
            }
            
            i += 1;
        }

        if !is_same_level {
            report.write(format!("\n</details>\n"));
        }
    }

    fn build_tree(sequences: HashMap<Vec<usize>, References>) -> TreeNode<usize, References>
    {
        info!("Building tree");

        let now = std::time::Instant::now();

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
    
        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len()));

        info!("Tree built and sorted in {} ms", now.elapsed().as_millis());

        return tree;
    }

    fn write_report(&mut self, tree: TreeNode<usize, References>) -> Result<(), HRESULT>
    {
        info!("Building report");

        let now = std::time::Instant::now();

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
        // print(&tree, 0, &|node: &TreeNode<ClassID, References>| format!("{} [inc:{}, exc:{:?}]",  self.clr().get_class_name(node.key), node.get_inclusive_value(), node.value));

        let nb_classes = tree.children.len();
        let nb_objects: usize = tree.children.iter().map(|x| x.get_inclusive_value().0.len()).sum();

        let mut report = self.session_info.create_report("summary.html".to_owned());

        report.write_line(format!("<h2>GC Survivors Report</h2>"));
        report.write_line(format!("<h3>{nb_objects} surviving objects of {nb_classes} classes..</h3>"));

        for tree_node in tree.children {
            self.print_html(&tree_node, false, &mut report);
        }

        info!("Report written in {} ms", now.elapsed().as_millis());

        Ok(())
    }

    // Post-process tracked persisting references
    fn build_sequences(&mut self) -> HashMap<Vec<usize>, References>
    {
        info!("Building graph of surviving references... {} objects to process", self.object_to_referencers.len());

        let now = std::time::Instant::now();

        let mut sequences: HashMap<Vec<ClassID>, References> = HashMap::new();

        for object in self.object_to_referencers.iter() {
            let info = self.clr();
            let object_id = object.key().clone();
            let size = info.get_object_size_2(object_id).unwrap_or(0);
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
                sequences.entry(branch)
                    .and_modify(|referencers| {referencers.0.insert(object_id.clone(), size);})
                    .or_insert(References(HashMap::from([(object_id.clone(), size)])));
            }
        }

        info!("Graph built in {} ms", now.elapsed().as_millis());

        sequences
    }
    
}

impl CorProfilerCallback for GCSurvivorsProfiler
{
    fn object_references(&mut self, object_id: ffi::ObjectID, class_id: ffi::ClassID, object_ref_ids: &[ffi::ObjectID]) -> Result<(), HRESULT>
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

        // Also add this object, with no referencers, just in case this object isn't referenced 
        self.object_to_referencers.insert(object_id, vec![]);

        Ok(())
    }
}

impl CorProfilerCallback2 for GCSurvivorsProfiler
{
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), HRESULT> {
        info!("garbage_collection_started on gen {} for reason {:?}", ClrProfilerInfo::get_gc_gen(&generation_collected), reason);
        
        if reason == ffi::COR_PRF_GC_REASON::COR_PRF_GC_INDUCED {
            self.is_triggered_gc.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), HRESULT>
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

        let sequences = self.build_sequences();
        let tree = Self::build_tree(sequences);
        let _ = self.write_report(tree);
        
        // We're done, we can detach :)
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();

        Ok(())
    }
}

impl CorProfilerCallback3 for GCSurvivorsProfiler
{
    fn initialize_for_attach(&mut self, profiler_info: ClrProfilerInfo, client_data: *const std::os::raw::c_void, client_data_length: u32) -> Result<(), HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), HRESULT>
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
        detach_after_duration::<GCSurvivorsProfiler>(&self, 320);

        Ok(())
    }
}

impl CorProfilerCallback4 for GCSurvivorsProfiler {}
impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}