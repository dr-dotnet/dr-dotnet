use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::BuildHasherDefault;
use std::ops::AddAssign;
use std::process::Child;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use dashmap::DashMap;
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
    root_objects: Vec<ObjectID>,
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
            uuid: "805A307B-061C-47F3-9B30-F795C3186E86".to_owned(),
            name: "GC Survivors (from roots)".to_owned(),
            description: "Alternative method using roots.".to_owned(),
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
    fn fill_tree(&self, node: &mut TreeNode<ClassID, References>, depth: usize, max_depth: usize, minimum_bytes: usize) { // MAYBE WE MISSE SOMETHING HERE
        let mut node_map: HashMap<ClassID, HashMap<ObjectID, usize>> = HashMap::default();
   
        if depth > max_depth {
            // Keep this node as-is, don't drill further
            return;
        }

        let info = self.clr();

        let mut references = Vec::<ObjectID>::new();

        if let Some(objects) = &node.value {
            for (object_id, size) in &objects.0 {
                references.clear();
                // We must pass this data as a pointer for callback to mutate it with actual object references ids
                let references_ptr_c = &references as *const Vec<ObjectID> as *mut std::ffi::c_void;
    
                let _ = info.enumerate_object_references(
                    *object_id, 
                    crate::utils::enum_references_callback,
                    references_ptr_c
                );    
    
                if references.len() == 0 {
                    // We reached the end of a path, copy the branch and add it to our branches, but only if final object lies in gen 2
                } else {
                    // Push new referencers if we are within our allowed depth
                    for i in 0..references.len() {
                        if let Ok(class_id) = info.get_class_from_object(references[i]) {

                            let size = info.get_object_size_2(references[i]).unwrap_or(0);

                            node_map.entry(class_id)
                                .and_modify(|roots| {
                                    roots.insert(references[i], size);
                                })
                                .or_insert_with(|| {
                                    let mut roots: HashMap<ObjectID, usize> = HashMap::default();
                                    roots.insert(references[i], size);
                                    roots
                                });
    
                        }
                    }
                }
            }
        }

        for (class_id, roots) in node_map {
            // let total_retained_bytes: usize = roots.values().sum();
            // if total_retained_bytes < 0 {
            //     // Skip
            //     continue;
            // }

            // in fact this method is garbage because we can't really discard at this stage, we don't know yet what the final retained bytes is
            let total_retained_objects: usize = roots.values().len();
            if total_retained_objects < 10 {
                // Skip
                continue;
            }

            let mut new_node = TreeNode { key: class_id, value: Some(References { 0: roots }), children: Vec::new() };
            self.fill_tree(&mut new_node, depth + 1, max_depth, minimum_bytes);

            node.children.push(new_node);
        }
    }

   // Post-process tracked persisting references
    fn build_sequences(&mut self) -> TreeNode<ClassID, References>
    {
        info!("Building graph of surviving references... {} roots to process", self.root_objects.len());

        let now = std::time::Instant::now();

        let info = self.clr();

        let mut tree = TreeNode::new(0);

        let mut node_map: HashMap<ClassID, HashMap<ObjectID, usize>> = HashMap::default();
   
        for object_id in self.root_objects.iter() {
            if let Ok(class_id) = info.get_class_from_object(*object_id) {
                node_map.entry(class_id)
                    .and_modify(|roots| {
                        roots.insert(*object_id, 0);
                    })
                    .or_insert_with(|| {
                        let mut roots: HashMap<ObjectID, usize> = HashMap::default();
                        roots.insert(*object_id, 0);
                        roots
                    });
            }
        }

        for (class_id, roots) in node_map {
            // let total_retained_bytes: usize = roots.values().sum();
            // if total_retained_bytes < 0 {
            //     // Skip
            //     continue;
            // }

            let total_retained_objects: usize = roots.values().len();
            if total_retained_objects < 10 {
                // Skip
                continue;
            }

            let mut new_node = TreeNode { key: class_id, value: Some(References { 0: roots }), children: Vec::new() };
            self.fill_tree(&mut new_node, 0, 10, 1000);

            tree.children.push(new_node);
        }

        info!("Graph built in {} ms", now.elapsed().as_millis());

        tree
    }

    fn is_gen_2(info: &ClrProfilerInfo, object_id: usize) -> bool {
        info.get_object_generation(object_id).map_or(false, |gen_info| gen_info.generation == ffi::COR_PRF_GC_GENERATION::COR_PRF_GC_GEN_2)
    }

    fn build_tree(&self, tree: &mut TreeNode<usize, References>)
    {
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

impl CorProfilerCallback for GCSurvivorsProfiler {}

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

        let mut tree = self.build_sequences();
        self.build_tree(&mut tree);
        let _ = self.write_report(tree);
        
        // We're done, we can detach :)
        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();

        Ok(())
    }

    fn root_references_2(
        &mut self,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        root_ids: &[UINT_PTR], // TODO: Maybe this should be a single array of some struct kind.
    ) -> Result<(), HRESULT> {

        if !self.is_triggered_gc.load(Ordering::Relaxed) {
            error!("Early return of garbage_collection_finished because GC wasn't forced yet");
            // Early return if we received an event before the forced GC started
            return Ok(());
        }

        for root_id in root_ref_ids {
            self.root_objects.push(*root_id);
        }
        
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