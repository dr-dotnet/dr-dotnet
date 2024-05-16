use deepsize::DeepSizeOf;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::BuildHasherDefault;
use std::ops::AddAssign;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use thousands::{digits, Separable, SeparatorPolicy};

use crate::api::*;
use crate::ffi::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{CachedNameResolver, NameResolver, SimpleHasher, TreeNode};

#[derive(Default)]
pub struct GCSurvivorsProfiler {
    name_resolver: CachedNameResolver,
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    root_objects: Vec<ObjectID>,
    root_kinds: HashMap<ObjectID, COR_PRF_GC_ROOT_KIND, BuildHasherDefault<SimpleHasher>>,
    is_relevant_gc: AtomicBool,
}

#[derive(Clone, Default, Debug)]
pub struct References(HashMap<ObjectID, usize, BuildHasherDefault<SimpleHasher>>);

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
            groups: &[3],
            digits: digits::ASCII_DECIMAL,
        };

        let nb_objects = self.0.len();
        let total_size: usize = self.0.values().sum();

        let total_size_str = if total_size > 0 {
            total_size.separate_by_policy(policy)
        } else {
            "???".to_string()
        };
        let nb_objects_str = nb_objects.separate_by_policy(policy);

        write!(f, "{nb_objects_str} / {total_size_str} B")
    }
}

impl Profiler for GCSurvivorsProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A307B-061C-47F3-9B30-F795C3186E86".to_owned(),
            name: "List GC survivors".to_owned(),
            description: "Perform a full blocking garbage collection and list the objects that survived it, grouped by type and sorted by size or count of retained objects. This helps to identify memory leaks, understand the retention paths of objects in memory and track down ephemeral objects that usually lengthen the time to perform a garbage collection.".to_owned(),
            parameters: vec![
                ProfilerParameter::define(
                    "Sort by size",
                    "sort_by_size",
                    false,
                    "If true, sort the results by inclusive size (bytes). Otherwise, sort by (inclusive) count of retained object. Disable this option to improve performance.",
                ),
                ProfilerParameter::define(
                    "Retained references threshold",
                    "retained_references_threshold",
                    100,
                    "Threshold of number of retained references by a node to ignore it. This helps to reduce the tree size, improving performance and readability.",
                ),
                ProfilerParameter::define(
                    "Retained bytes threshold",
                    "retained_bytes_threshold",
                    10000,
                    "Threshold of number of retained bytes by a node to ignore it. This helps to reduce the tree size, improving performance and readability.",
                ),
                ProfilerParameter::define(
                    "Maximum depth",
                    "max_depth",
                    4,
                    "The maximum depth while drilling through retention paths. This helps to reduce the tree size, improving performance and readability.",
                ),
            ],
            ..std::default::Default::default()
        };
    }
}

impl GCSurvivorsProfiler {
    fn gather_references_recursive(
        info: &ClrProfilerInfo,
        node: &mut TreeNode<ClassID, References>,
        depth: usize,
        max_depth: usize,
        retained_references_threshold: usize,
        retained_bytes_threshold: usize,
    ) {
        if depth > max_depth {
            // Keep this node as-is, don't drill further
            return;
        }

        if let Some(references) = &node.value {
            let reference_object_ids = Vec::<ObjectID>::new();

            // For each object instance, enumerate its references
            for &object_id in references.0.keys() {
                if object_id == 0 {
                    continue;
                }
                // We must pass this data as a pointer for callback to mutate it with actual object references ids
                let references_ptr_c = &reference_object_ids as *const Vec<ObjectID> as *mut std::ffi::c_void;
                let _ = info.enumerate_object_references(object_id, crate::utils::enum_references_callback, references_ptr_c);
            }

            Self::build_tree_nodes(
                info,
                &reference_object_ids,
                node,
                depth,
                max_depth,
                retained_references_threshold,
                retained_bytes_threshold,
            );
        }
    }

    fn build_tree_nodes(
        clr: &ClrProfilerInfo,
        reference_object_ids: &Vec<ObjectID>,
        node: &mut TreeNode<usize, References>,
        depth: usize,
        max_retention_depth: usize,
        retained_references_threshold: usize,
        retained_bytes_threshold: usize
    ) {
        let mut map: HashMap<ClassID, References> = HashMap::new();

        // Fill a map of class_id -> references
        // This will allow us to group references by class and ease the creation of child nodes in the tree
        for &object_id in reference_object_ids.iter() {
            if object_id == 0 {
                continue;
            }
            let refs = map.entry(clr.get_class_from_object(object_id).unwrap()).or_insert(References::default());
            let size = clr.get_object_size_2(object_id).unwrap_or(0);
            refs.0.insert(object_id, size);
        }

        // For each class_id, create a child node and fill it with references
        for (class_id, refs) in map {
            let mut child_node = TreeNode::new(class_id);
            child_node.value = Some(refs);
            Self::gather_references_recursive(
                clr,
                &mut child_node,
                depth + 1,
                max_retention_depth,
                retained_references_threshold,
                retained_bytes_threshold,
            );

            // Discard the branch if it doesn't meet the thresholds
            // This will save memory, lower CPU usage when sorting the tree and lighten the final report
            let inclusive_values = child_node.get_inclusive_value().0;
            let mut discard_branch = inclusive_values.len() < retained_references_threshold;
            discard_branch |= inclusive_values.values().sum::<usize>() < retained_bytes_threshold;
            if !discard_branch {
                node.children.push(child_node);
            }
        }
    }

    fn build_tree(&mut self) -> TreeNode<ClassID, References> {
        info!("Building tree of surviving references from {} roots...", self.root_objects.len());

        let now = std::time::Instant::now();

        let mut tree = TreeNode::new(0);

        let max_retention_depth = self.session_info().get_parameter::<usize>("max_depth").unwrap();
        let retained_references_threshold = self.session_info().get_parameter::<usize>("retained_references_threshold").unwrap();
        let retained_bytes_threshold = self.session_info().get_parameter::<usize>("retained_bytes_threshold").unwrap();

        info!("Build root node refs...");
        let clr = self.clr();

        Self::build_tree_nodes(
            clr,
            &self.root_objects,
            &mut tree,
            0,
            max_retention_depth,
            retained_references_threshold,
            retained_bytes_threshold,
        );

        info!("Tree built in {} ms", now.elapsed().as_millis());

        tree
    }

    fn sort_tree(&self, tree: &mut TreeNode<usize, References>) {
        info!("Sorting tree...");

        let now = std::time::Instant::now();

        let sort_by_size = self.session_info().get_parameter::<bool>("sort_by_size").unwrap();

        let compare = &|a: &TreeNode<usize, References>, b: &TreeNode<usize, References>| {
            if sort_by_size {
                // Sorts by descending inclusive size
                b.get_inclusive_value()
                    .0
                    .values()
                    .sum::<usize>()
                    .cmp(&a.get_inclusive_value().0.values().sum::<usize>())
            } else {
                // Sorts by descending inclusive count
                b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len())
            }
        };

        // Start by sorting the tree "roots" (only the first level of childrens)
        tree.children.sort_by(compare);

        // Then sort the whole tree (all levels of childrens)
        tree.sort_by_iterative(compare);

        info!("Tree sorted in {} ms", now.elapsed().as_millis());
    }

    fn write_report(&mut self, tree: TreeNode<usize, References>) -> Result<(), HRESULT> {
        info!("Writing report...");

        let now = std::time::Instant::now();

        let nb_classes = tree.children.len();
        let nb_objects: usize = tree.children.iter().map(|x| x.get_inclusive_value().0.len()).sum();

        let mut report = self.session_info.create_report("summary.html".to_owned());

        report.write_line(format!("<h2>GC Survivors Report</h2>"));
        report.write_line(format!("The GC survivors report displays a tree of objects that survived the last garbage collection. The first level of the tree represents the roots of the objects grouped by typed. Each node of the tree represents a class of objects that are retained by its parent node, and so on until there are no more references to follow or maximum depth is reached. The tree is sorted by the number of retained objects or bytes, depending on the selected parameter."));

        // Quick legend
        report.write_line(format!(" \
            <details> \
                <summary> \
                    <code>MyRootObject</code> \
                    <div class=\"chip\"><span>retained objects including all children / retained bytes including all children</span><i class=\"material-icons\">radio_button_checked</i></div> \
                    <div class=\"chip\"><span>retained objects / retained bytes</span><i class=\"material-icons\">radio_button_unchecked</i></div> \
                    <div class=\"chip\"><span>other</span><i class=\"material-icons\">help</i></div> \
                    <div class=\"chip\"><span>finalizer</span><i class=\"material-icons\">auto_delete</i></div> \
                    <div class=\"chip\"><span>handle</span><i class=\"material-icons\">flag</i></div> \
                    <div class=\"chip\"><span>stack</span><i class=\"material-icons\">segment</i></div> \
                </summary> \
                <ul><li> \
                    <code>MySurvivingObject</code> \
                    <div class=\"chip\"><span>retained objects including all children / retained bytes including all children</span><i class=\"material-icons\">radio_button_checked</i></div> \
                    <div class=\"chip\"><span>retained objects / retained bytes</span><i class=\"material-icons\">radio_button_unchecked</i></div> \
                </li></ul> \
            </details>"));
        
        report.write_line(format!("<h3>Retention Tree</h3>"));
        report.write_line(format!("<h4>{nb_objects} surviving objects of {nb_classes} classes</h4>"));

        for tree_node in tree.children {
            self.print_html(&tree_node, 0, &mut report);
        }

        info!("Report written in {} ms", now.elapsed().as_millis());

        Ok(())
    }

    fn print_html(&self, tree: &TreeNode<ClassID, References>, depth: usize, report: &mut Report) {

        let binding = References::default();
        let references_exlusive = match &tree.value {
            None => &binding,
            Some(refs) => refs
        };
        let references_inclusive = &tree.get_inclusive_value();

        if tree.key == 0 {
            report.write_line(format!("Path truncated because of depth limit reached"));
            return;
        }

        let mut class_name = self.name_resolver.get_class_name(tree.key);
        let escaped_class_name = html_escape::encode_text(&mut class_name);

        let has_children = tree.children.len() > 0;

        let mut line: String = format!("<code>{escaped_class_name}</code> \
            <div class=\"chip\"><span>{references_inclusive}</span><i class=\"material-icons\">radio_button_checked</i></div> \
            <div class=\"chip\"><span>{references_exlusive}</span><i class=\"material-icons\">radio_button_unchecked</i></div>");

        if depth == 0 {
            // Find index of first item with id 42
            let mut count_per_kind = HashMap::new();
            for (object_id, size) in &tree.value.as_ref().unwrap().0 {
                let kind = self.root_kinds.get(&object_id).unwrap();
                let count = count_per_kind.entry(kind).or_insert(0);
                *count += 1;
            }
            for (kind, count) in count_per_kind {
                let kind_icon = match kind {
                    ffi::COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_STACK => "segment",
                    ffi::COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_FINALIZER => "auto_delete",
                    ffi::COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_HANDLE => "flag",
                    ffi::COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_OTHER => "help"
                };
                line = format!("{line}<div class=\"chip\"><span>{count}</span><i class=\"material-icons\">{kind_icon}</i></div>");
            }
        }

        if has_children {
            report.write_line(format!("<details><summary>{line}</summary>"));
            report.write_line(format!("<ul>"));
            for child in &tree.children {
                self.print_html(child, depth + 1, report);
            }
            report.write_line(format!("</ul>"));
            report.write_line(format!("</details>"));
        } else {
            report.write_line(format!("<li>{line}</li>"));
        }
    }
}

impl CorProfilerCallback for GCSurvivorsProfiler {}

impl CorProfilerCallback2 for GCSurvivorsProfiler {
    fn garbage_collection_started(&mut self, generation_collected: &[ffi::BOOL], reason: ffi::COR_PRF_GC_REASON) -> Result<(), HRESULT> {
        let gen = ClrProfilerInfo::get_gc_gen(&generation_collected);

        info!("garbage_collection_started on gen {} for reason {:?}", gen, reason);

        if reason == ffi::COR_PRF_GC_REASON::COR_PRF_GC_INDUCED {
            debug!("induced gc!");
            self.is_relevant_gc.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), HRESULT> {
        info!("garbage_collection_finished");

        if !self.is_relevant_gc.load(Ordering::Relaxed) {
            error!("Early return of garbage_collection_finished because GC wasn't forced yet");
            // Early return if we received an event before the forced GC started
            return Ok(());
        }

        // Disable profiling to free some resources
        match self
            .clr()
            .set_event_mask_2(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_NONE, ffi::COR_PRF_HIGH_MONITOR::COR_PRF_HIGH_MONITOR_NONE)
        {
            Ok(_) => (),
            Err(hresult) => error!("Error setting event mask: {:?}", hresult),
        }

        info!("Deep size of roots: {} bytes", self.root_objects.deep_size_of());

        let mut tree = self.build_tree();
        self.sort_tree(&mut tree);
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
        if !self.is_relevant_gc.load(Ordering::Relaxed) {
            warn!("Early return of garbage_collection_finished because GC wasn't forced yet");
            // Early return if we received an event before the forced GC started
            return Ok(());
        }

        for i in 0..root_ref_ids.len() {
            let root_id = root_ref_ids[i];
            let root_kind = root_kinds[i];
            //let root_flag = root_flags[i];

            self.root_objects.push(root_id);
            self.root_kinds.insert(root_id, root_kind);
        }

        Ok(())
    }
}

impl CorProfilerCallback3 for GCSurvivorsProfiler {
    fn initialize_for_attach(
        &mut self,
        profiler_info: ClrProfilerInfo,
        client_data: *const std::os::raw::c_void,
        client_data_length: u32,
    ) -> Result<(), HRESULT> {
        self.init(ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC, None, profiler_info, client_data, client_data_length)
    }

    fn profiler_attach_complete(&mut self) -> Result<(), HRESULT> {
        self.name_resolver = CachedNameResolver::new(self.clr().clone());

        // The ForceGC method must be called only from a thread that does not have any profiler callbacks on its stack.
        // https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-forcegc-method
        let clr = self.clr().clone();

        let _ = thread::spawn(move || {
            debug!("Force GC");

            match clr.force_gc() {
                Ok(_) => debug!("GC Forced!"),
                Err(hresult) => error!("Error forcing GC: {:?}", hresult),
            };
        })
        .join();

        // Security timeout
        detach_after_duration::<GCSurvivorsProfiler>(&self, 320);

        Ok(())
    }

    fn profiler_detach_succeeded(&mut self) -> Result<(), ffi::HRESULT> {
        self.session_info.finish();
        Ok(())
    }
}

impl CorProfilerCallback4 for GCSurvivorsProfiler {}
impl CorProfilerCallback5 for GCSurvivorsProfiler {}
impl CorProfilerCallback6 for GCSurvivorsProfiler {}
impl CorProfilerCallback7 for GCSurvivorsProfiler {}
impl CorProfilerCallback8 for GCSurvivorsProfiler {}
impl CorProfilerCallback9 for GCSurvivorsProfiler {}
