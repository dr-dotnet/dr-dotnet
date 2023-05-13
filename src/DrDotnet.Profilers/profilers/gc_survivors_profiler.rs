use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use deepsize::DeepSizeOf;
use thousands::{digits, Separable, SeparatorPolicy};

use crate::ffi::*;
use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{TreeNode, CachedNameResolver, NameResolver};

#[derive(Default)]
pub struct GCSurvivorsProfiler {
    name_resolver: CachedNameResolver,
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_referencers: HashMap<ObjectID, Vec<ObjectID>>,
    is_triggered_gc: AtomicBool,
    gc_start_time: Option<Instant>
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
            description: "Todo.".to_owned(),
            is_released: true,
            parameters: vec![
                ProfilerParameter { 
                    name: "Sort by size".to_owned(),
                    key: "sort_by_size".to_owned(),
                    description: "If true, sort the results by inclusive size (bytes). Otherwise, sort by inclusive instances count.".to_owned(),
                    type_: ParameterType::BOOLEAN.into(),
                    value: "true".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter {
                    name: "Sort multi-threaded".to_owned(),
                    key: "sort_multithreaded".to_owned(),
                    description: "If true, sort the results with a multi-threaded methode. Otherwise, sort with an iterative method.".to_owned(),
                    type_: ParameterType::BOOLEAN.into(),
                    value: "false".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter { 
                    name: "Maximum types to display".to_owned(),
                    key: "max_types_display".to_owned(),
                    description: "The maximum number of types to display in the report".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "1000".to_owned(),
                    ..std::default::Default::default()
                },
                ProfilerParameter { 
                    name: "Maximum depth".to_owned(),
                    key: "max_retention_depth".to_owned(),
                    description: "The maximum depth while drilling through retention paths".to_owned(),
                    type_: ParameterType::INT.into(),
                    value: "4".to_owned(),
                    ..std::default::Default::default()
                }
            ],
            ..std::default::Default::default()
        }
    }
}

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
                    } else if current_id.1 < max_depth {
                        // Push new referencers if we are within our allowed depth
                        for i in 0..referencers.len() {
                            stack.push((referencers[i], current_id.1 + 1));
                        }
                    }
                    else {
                        // If max depth is reached, we push a 0 terminated branch to indicate that branch is truncated
                        let mut deep_branch = branch.clone();
                        deep_branch.push(0);
                        branches.push(deep_branch);
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

    // Post-process tracked persisting references
    fn build_sequences(&mut self) -> HashMap<Vec<usize>, References>
    {
        info!("Building graph of surviving references... {} objects to process", self.object_to_referencers.len());

        let now = std::time::Instant::now();

        let mut sequences: HashMap<Vec<ClassID>, References> = HashMap::new();

        let info = self.clr();

        let max_retention_depth = self.session_info().get_parameter::<usize>("max_retention_depth").unwrap();

        for object in self.object_to_referencers.iter() {
            
            let object_id: ObjectID = object.0.clone();

            if !Self::is_gen_2(info, object_id) {
                continue;
            }

            let size = info.get_object_size_2(object_id).unwrap_or(0);

            for branch in self.append_references(info, object_id, max_retention_depth) {
                sequences.entry(branch)
                    .and_modify(|referencers| {referencers.0.insert(object_id.clone(), size);})
                    .or_insert(References(HashMap::from([(object_id.clone(), size)])));
            }
        }

        info!("Graph built in {} ms", now.elapsed().as_millis());

        sequences
    }

    fn is_gen_2(info: &ClrProfilerInfo, object_id: usize) -> bool {
        info.get_object_generation(object_id).map_or(false, |gen_info| gen_info.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2)
    }

    fn build_tree(&self, sequences: &mut HashMap<Vec<usize>, References>) -> TreeNode<usize, References>
    {
        info!("Building tree");

        let now = std::time::Instant::now();

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);

        sequences.clear();

        info!("Tree built in {} ms", now.elapsed().as_millis());

        info!("Sorting tree");

        let now = std::time::Instant::now();
    
        let sort_by_size = self.session_info().get_parameter::<bool>("sort_by_size").unwrap();
        
        let compare = &|a:&TreeNode<usize, References>, b:&TreeNode<usize, References>| {
            if sort_by_size {
                // Sorts by descending inclusive size
                b.get_inclusive_value().0.values().sum::<usize>().cmp(&a.get_inclusive_value().0.values().sum::<usize>())
            } else {
                // Sorts by descending inclusive count
                b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len())
            }
        };
        
        // Start by sorting the tree "roots" (only the first level of childrens)
        tree.children.sort_by(compare);

        // Keep the first "max_types_display" roots
        let mut max_types_display = self.session_info().get_parameter::<usize>("max_types_display").unwrap();
        max_types_display = min(max_types_display, tree.children.len());
        tree.children.truncate(max_types_display);

        // Then sort the whole tree (all levels of childrens)
        let sort_multithreaded = self.session_info().get_parameter::<bool>("sort_multithreaded").unwrap();
        if sort_multithreaded {
            tree.sort_by_multithreaded(compare);
        }
        else {
            tree.sort_by_iterative(compare);
        }
 
        info!("Tree sorted in {} ms", now.elapsed().as_millis());

        return tree;
    }

    fn write_report(&mut self, tree: TreeNode<usize, References>) -> Result<(), HRESULT>
    {
        info!("Building report");

        let now = std::time::Instant::now();

        let nb_classes = tree.children.len();
        let nb_objects: usize = tree.children.iter().map(|x| x.get_inclusive_value().0.len()).sum();

        let mut report = self.session_info.create_report("summary.html".to_owned());

        report.write_line(format!("<h2>GC Survivors Report</h2>"));
        report.write_line(format!("<h3>{nb_objects} surviving objects of {nb_classes} classes</h3>"));

        for tree_node in tree.children {
            self.print_html(&tree_node, &mut report);
        }

        info!("Report written in {} ms", now.elapsed().as_millis());

        Ok(())
    }

    fn print_html(&self, tree: &TreeNode<ClassID, References>, report: &mut Report)
    {
        let refs = &tree.get_inclusive_value();

        if tree.key == 0 { 
            report.write_line(format!("Path truncated because of depth limit reached"));
            return;
        }
        
        let mut class_name = self.name_resolver.get_class_name(tree.key);
        let escaped_class_name = html_escape::encode_text(&mut class_name);

        let has_children = tree.children.len() > 0;

        if has_children {
            report.write_line(format!("<details><summary><span>{refs}</span><code>{escaped_class_name}</code></summary>"));
            report.write_line(format!("<ul>"));
            for child in &tree.children {
                self.print_html(child, report);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li><span>{refs}</span><code>{escaped_class_name}</code></li>"));
        }
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

        // If an object has no references and is not gen 2, we can discard it, because it means it will never reference any
        // other gen 2 object, which is actually what we are looking for
        if object_ref_ids.is_empty() && !Self::is_gen_2(self.clr(), object_id) {
            return Ok(());
        }

        // Create dependency tree, but from object to referencers, instead of object to its references.
        // This is usefull for being able to browse from any object back to its roots.
        for object_ref_id in object_ref_ids {
            self.object_to_referencers.entry(*object_ref_id).or_insert(Vec::new()).push(object_id);
        }

        // Also add this object, with no referencers, just in case this object isn't referenced 
        self.object_to_referencers.entry(object_id).or_insert(Vec::new());

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

        self.gc_start_time = Some(std::time::Instant::now());

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

        info!("Garbage collection done in {} ms", self.gc_start_time.unwrap().elapsed().as_millis());

        // Disable profiling to free some resources
        match self.clr().set_event_mask(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE) {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:?}", hresult)
        }

        // Before Option<T> : 1555856 bytes
        info!(">>> Deep size of object references: {} bytes", self.object_to_referencers.deep_size_of());

        let mut sequences = self.build_sequences();
        let tree = self.build_tree(&mut sequences);
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
        self.name_resolver = CachedNameResolver::new(self.clr().clone());

        // The ForceGC method must be called only from a thread that does not have any profiler callbacks on its stack. 
        // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-forcegc-method
        let clr = self.clr().clone();

        let _ = thread::spawn(move || {
            debug!("Force GC");
            
            match clr.force_gc() {
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