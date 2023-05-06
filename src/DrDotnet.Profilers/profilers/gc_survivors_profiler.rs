use std::cmp::{max, min};
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ops::AddAssign;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use thousands::{digits, Separable, SeparatorPolicy};

use crate::ffi::*;
use crate::api::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{TreeNode, CachedNameResolver, NameResolver, SimpleHasher};

#[derive(Default)]
pub struct GCSurvivorsProfiler {
    name_resolver: CachedNameResolver,
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    object_to_referencers: HashMap<ObjectID, Option<Vec<ObjectID>>, BuildHasherDefault::<SimpleHasher>>,
    is_triggered_gc: AtomicBool
}

#[derive(Clone, Default, Debug)]
pub struct Roots {
    objects_to_size: HashMap<ObjectID, usize, BuildHasherDefault::<SimpleHasher>>,
}

// Implement AddAssign for get_inclusive_value to be usable
impl AddAssign<&Roots> for Roots {
    fn add_assign(&mut self, other: &Self) {
        self.objects_to_size.extend(&other.objects_to_size);
    }
}

impl Display for Roots {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let policy = SeparatorPolicy {
            separator: ",",
            groups:    &[3],
            digits:    digits::ASCII_DECIMAL,
        };
        
        let nb_objects = self.objects_to_size.len();
        let total_size: usize = self.objects_to_size.values().sum();

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
                    name: "Minimum bytes".to_owned(),
                    key: "minimum_bytes".to_owned(),
                    description: "The minimum retained bytes for a path to be displayed".to_owned(),
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
    fn fill_tree(&self, node: &mut TreeNode<ClassID, Roots>, retaining_objects: Vec<(ObjectID, ObjectID, usize)>, depth: usize, max_depth: usize, minimum_bytes: usize) { // MAYBE WE MISSE SOMETHING HERE
        let mut node_map: HashMap<ClassID, (HashMap<ObjectID, usize, BuildHasherDefault::<SimpleHasher>>, Vec<(ObjectID, ObjectID, usize)>)> = HashMap::default();
        let mut current_node_map = Roots::default(); 

        if depth > max_depth {
            // Keep this node as-is, don't drill further
            return;
        }

        let info = self.clr();

        // For each object retaining this node
        for (retaining_object_id, retained_object_id, retained_object_size) in &retaining_objects {
            // Check if there are object retaining this object
            if let Some(referencers) = self.object_to_referencers.get(retaining_object_id).unwrap_or(&None) {
                // For each object retaining this object
                for referencer in referencers {
                    if let Ok(class_id) = info.get_class_from_object(*referencer) {
                        node_map.entry(class_id)
                            .and_modify(|roots| {
                                roots.0.insert(*retained_object_id, *retained_object_size);
                                roots.1.push((*referencer, *retained_object_id, *retained_object_size));
                            })
                            .or_insert_with(|| {
                                let mut roots: HashMap<ObjectID, usize, BuildHasherDefault::<SimpleHasher>> = HashMap::default();
                                roots.insert(*retained_object_id, *retained_object_size);
                                (roots, vec![(*referencer, *retained_object_id, *retained_object_size)])
                            });
                    }
                }
            } else {
                // There is no object retaining this object, thus it is a root
                // In this case, it means it belongs to the current node
                current_node_map.objects_to_size.insert(*retained_object_id, *retained_object_size);
            }
        }

        node.value = if current_node_map.objects_to_size.len() > 0 { Some(current_node_map) } else { None };

        for (class_id, roots) in node_map {
            let total_retained_bytes: usize = roots.0.values().sum();
            if total_retained_bytes < minimum_bytes {
                // Skip
                continue;
            }

            let mut new_node = TreeNode { key: class_id, value: Some(Roots { objects_to_size: roots.0 }), children: Vec::new() };
            self.fill_tree(&mut new_node, roots.1, depth + 1, max_depth, minimum_bytes);

            node.children.push(new_node);
        }
    }

    // Post-process tracked persisting references
    fn build_tree(&mut self) -> TreeNode<ClassID, Roots>
    {
        info!("Building graph of surviving references... {} objects to process", self.object_to_referencers.len());

        let now = std::time::Instant::now();

        let info = self.clr();

        let max_retention_depth = self.session_info().get_parameter::<usize>("max_retention_depth").unwrap();
        let minimum_bytes = self.session_info().get_parameter::<usize>("minimum_bytes").unwrap();

        let mut tree: TreeNode<ClassID, Roots> = TreeNode::new(0);
        let mut node_map: HashMap<ClassID, (HashMap<ObjectID, usize, BuildHasherDefault::<SimpleHasher>>, Vec<(ObjectID, ObjectID, usize)>)> = HashMap::default();

        // For each object retaining this node
        for object_id in self.object_to_referencers.keys() {

            if !Self::is_gen_2(info, *object_id) {
                continue;
            }

            let size = info.get_object_size_2(*object_id).unwrap_or(0);

            if let Ok(class_id) = info.get_class_from_object(*object_id) {
                node_map.entry(class_id)
                    .and_modify(|roots| {
                        roots.0.insert(*object_id, size);
                        roots.1.push((*object_id, *object_id, size));
                    })
                    .or_insert_with(|| {
                        let mut roots: HashMap<ObjectID, usize, BuildHasherDefault::<SimpleHasher>> = HashMap::default();
                        roots.insert(*object_id, size);
                        (roots, vec![(*object_id, *object_id, size)])
                    });
            }
        }
  
        for (class_id, roots) in node_map {
            let total_retained_bytes: usize = roots.0.values().sum();
            if total_retained_bytes < minimum_bytes {
                // Skip
                continue;
            }

            let mut new_node = TreeNode { key: class_id, value: Some(Roots { objects_to_size: roots.0 }), children: Vec::new() };
            self.fill_tree(&mut new_node, roots.1, 1, max_retention_depth, minimum_bytes);

            tree.children.push(new_node);
        }

        info!("Graph built in {} ms", now.elapsed().as_millis());

        tree
    }

    fn is_gen_2(info: &ClrProfilerInfo, object_id: usize) -> bool {
        info.get_object_generation(object_id).map_or(false, |gen_info| gen_info.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2)
    }

    fn sort_tree(&self, tree: &mut TreeNode<usize, Roots>)
    {
        info!("Sorting tree");

        let now = std::time::Instant::now();
    
        let sort_by_size = self.session_info().get_parameter::<bool>("sort_by_size").unwrap();
        
        let compare = &|a:&TreeNode<usize, Roots>, b:&TreeNode<usize, Roots>| {
            if sort_by_size {
                // Sorts by descending inclusive size
                b.get_inclusive_value().objects_to_size.values().sum::<usize>().cmp(&a.get_inclusive_value().objects_to_size.values().sum::<usize>())
            } else {
                // Sorts by descending inclusive count
                b.get_inclusive_value().objects_to_size.len().cmp(&a.get_inclusive_value().objects_to_size.len())
            }
        };
        
        // Then sort the whole tree (all levels of childrens)
        let sort_multithreaded = self.session_info().get_parameter::<bool>("sort_multithreaded").unwrap();
        if sort_multithreaded {
            tree.sort_by_multithreaded(compare);
        }
        else {
            tree.sort_by_iterative(compare);
        }
 
        info!("Tree sorted in {} ms", now.elapsed().as_millis());
    }

    fn write_report(&mut self, tree: TreeNode<usize, Roots>) -> Result<(), HRESULT>
    {
        info!("Building report");

        let now = std::time::Instant::now();

        let nb_classes = tree.children.len();
        let nb_objects: usize = tree.children.iter().map(|x| x.get_inclusive_value().objects_to_size.len()).sum();

        let mut report = self.session_info.create_report("summary.html".to_owned());

        report.write_line(format!("<h2>GC Survivors Report</h2>"));
        report.write_line(format!("<h3>{nb_objects} surviving objects of {nb_classes} classes</h3>"));

        for tree_node in tree.children {
            self.print_html(&tree_node, &mut report);
        }

        info!("Report written in {} ms", now.elapsed().as_millis());

        Ok(())
    }

    fn print_html(&self, tree: &TreeNode<ClassID, Roots>, report: &mut Report)
    {
        let excl = match &tree.value {
            Some(roots) => roots.objects_to_size.len(),
            _ => 0
        };
        let refs = &tree.get_inclusive_value();

        if tree.key == 0 { 
            report.write_line(format!("Path truncated because of depth limit reached"));
            return;
        }
        
        let mut class_name = self.name_resolver.get_class_name(tree.key);
        let escaped_class_name = html_escape::encode_text(&mut class_name);

        let has_children = tree.children.len() > 0;

        if has_children {
            report.write_line(format!("<details><summary><span>{refs} / {excl}</span><code>{escaped_class_name}</code></summary>"));
            report.write_line(format!("<ul>"));
            for child in &tree.children {
                self.print_html(child, report);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li><span>{refs} / {excl}</span><code>{escaped_class_name}</code></li>"));
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

        // Create dependency tree, but from object to referencers, instead of object to its references.
        // This is usefull for being able to browse from any object back to its roots.
        for object_ref_id in object_ref_ids {
            self.object_to_referencers.entry(*object_ref_id)
                .and_modify(|e| e.get_or_insert_with(|| Vec::new()).push(object_id))
                .or_insert_with(|| Some(vec![object_id]));
        }

        // Also add this object, with no referencers, just in case this object isn't referenced 
        self.object_to_referencers.entry(object_id).or_insert(None);

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

        let mut tree = self.build_tree();
        self.sort_tree(&mut tree);
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