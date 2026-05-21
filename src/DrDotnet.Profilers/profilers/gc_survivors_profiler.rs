use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::BuildHasherDefault;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use thousands::{digits, Separable, SeparatorPolicy};

use crate::api::*;
use crate::ffi::*;
use crate::macros::*;
use crate::profilers::*;
use crate::session::Report;
use crate::utils::{enum_references_callback, CachedNameResolver, NameResolver, SimpleHasher, TreeNode};

type FastHasher = BuildHasherDefault<SimpleHasher>;
type FastMap<K, V> = HashMap<K, V, FastHasher>;
type FastSet<T> = HashSet<T, FastHasher>;

const SEPARATOR_POLICY: SeparatorPolicy = SeparatorPolicy {
    separator: ",",
    groups: &[3],
    digits: digits::ASCII_DECIMAL,
};

/// Inline-small list of referencers. Most live objects are referenced by a
/// single other object, so we keep that case inline (no heap allocation, no
/// Vec header). Objects with many referencers spill to a boxed Vec.
enum Parents {
    None,
    One(ObjectID),
    Many(Box<Vec<ObjectID>>),
}

impl Default for Parents {
    fn default() -> Self {
        Parents::None
    }
}

impl Parents {
    fn push(&mut self, parent: ObjectID) {
        match self {
            Parents::None => *self = Parents::One(parent),
            Parents::One(existing) => {
                *self = Parents::Many(Box::new(vec![*existing, parent]));
            }
            Parents::Many(v) => v.push(parent),
        }
    }
}

enum ParentsIter<'a> {
    Empty,
    Single(Option<ObjectID>),
    Slice(std::slice::Iter<'a, ObjectID>),
}

impl<'a> Iterator for ParentsIter<'a> {
    type Item = ObjectID;
    fn next(&mut self) -> Option<ObjectID> {
        match self {
            ParentsIter::Empty => None,
            ParentsIter::Single(opt) => opt.take(),
            ParentsIter::Slice(iter) => iter.next().copied(),
        }
    }
}

impl<'a> IntoIterator for &'a Parents {
    type Item = ObjectID;
    type IntoIter = ParentsIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Parents::None => ParentsIter::Empty,
            Parents::One(p) => ParentsIter::Single(Some(*p)),
            Parents::Many(v) => ParentsIter::Slice(v.iter()),
        }
    }
}

/// Lean per-object record in the reverse reference graph.
struct ObjectInfo {
    class_id: ClassID,
    size: u32,
    /// Direct referencers (i.e. parents in the retention graph).
    parents: Parents,
}

#[derive(Default, Clone, Copy)]
struct ClassTotals {
    count: u64,
    total_size: u64,
}

#[derive(Default)]
struct HeapGraph {
    objects: FastMap<ObjectID, ObjectInfo>,
    class_totals: FastMap<ClassID, ClassTotals>,
}

/// Per-node aggregate carried on every `TreeNode` in a retention tree.
/// `instances` holds the concrete object IDs that belong to this node's class
/// AND are reachable along the current retention path; aggregates are derived
/// from that set at build time so sort comparisons are O(1).
#[derive(Default, Clone)]
struct NodeAgg {
    instances: FastSet<ObjectID>,
    self_count: u64,
    self_size: u64,
}

#[derive(Default)]
pub struct GCSurvivorsProfiler {
    name_resolver: CachedNameResolver,
    clr_profiler_info: ClrProfilerInfo,
    session_info: SessionInfo,
    roots: Vec<ObjectID>,
    root_kinds: FastMap<ObjectID, COR_PRF_GC_ROOT_KIND>,
    is_relevant_gc: AtomicBool,
}

impl Profiler for GCSurvivorsProfiler {
    profiler_getset!();

    fn profiler_info() -> ProfilerInfo {
        return ProfilerInfo {
            uuid: "805A307B-061C-47F3-9B30-F795C3186E86".to_owned(),
            name: "List GC survivors".to_owned(),
            description: "Perform a full blocking garbage collection and list the objects that survived it, grouped by class. For each class the report shows the retention chain going upward toward the GC roots (what is keeping these instances alive).".to_owned(),
            parameters: vec![
                ProfilerParameter::define(
                    "Sort by size",
                    "sort_by_size",
                    false,
                    "If true, sort the results by retained bytes. Otherwise, sort by retained instance count.",
                ),
                ProfilerParameter::define(
                    "Retained instances threshold",
                    "retained_references_threshold",
                    100,
                    "Minimum count of retained instances for a class branch to be displayed.",
                ),
                ProfilerParameter::define(
                    "Retained bytes threshold",
                    "retained_bytes_threshold",
                    10000,
                    "Minimum retained bytes for a class branch to be displayed.",
                ),
                ProfilerParameter::define(
                    "Maximum depth",
                    "max_depth",
                    4,
                    "Maximum depth when walking parents back toward roots.",
                ),
            ],
            ..std::default::Default::default()
        };
    }
}

impl GCSurvivorsProfiler {
    /// BFS from the captured roots; build a reverse reference graph plus
    /// per-class totals. Each reachable object is visited exactly once.
    fn build_heap_graph(&self) -> HeapGraph {
        let started_at = std::time::Instant::now();
        let clr = self.clr();
        let mut graph = HeapGraph::default();

        // Seed BFS from deduplicated roots.
        let mut queue: VecDeque<ObjectID> = VecDeque::with_capacity(self.roots.len());
        for &root in &self.roots {
            if root == 0 {
                continue;
            }
            if Self::observe(&mut graph, clr, root) {
                queue.push_back(root);
            }
        }

        // Reusable scratch buffer for the FFI callback.
        let mut children: Vec<ObjectID> = Vec::with_capacity(16);

        while let Some(parent) = queue.pop_front() {
            children.clear();
            let children_ptr = &mut children as *mut Vec<ObjectID> as *mut std::ffi::c_void;
            let _ = clr.enumerate_object_references(parent, enum_references_callback, children_ptr);

            for &child in children.iter() {
                if child == 0 {
                    continue;
                }
                let was_new = Self::observe(&mut graph, clr, child);
                // Record the reverse edge unconditionally (every parent counts).
                if let Some(info) = graph.objects.get_mut(&child) {
                    info.parents.push(parent);
                }
                if was_new {
                    queue.push_back(child);
                }
            }
        }

        info!(
            "Reverse graph built: {} objects across {} classes in {} ms",
            graph.objects.len(),
            graph.class_totals.len(),
            started_at.elapsed().as_millis()
        );
        graph
    }

    /// Insert an object into the graph if not already present. Returns true
    /// when the object was newly added (caller should push to BFS queue).
    fn observe(graph: &mut HeapGraph, clr: &ClrProfilerInfo, obj: ObjectID) -> bool {
        match graph.objects.entry(obj) {
            Entry::Occupied(_) => false,
            Entry::Vacant(vac) => {
                let class_id = clr.get_class_from_object(obj).unwrap_or(0);
                let size = clr.get_object_size_2(obj).unwrap_or(0) as u32;
                vac.insert(ObjectInfo {
                    class_id,
                    size,
                    parents: Parents::None,
                });
                let agg = graph.class_totals.entry(class_id).or_default();
                agg.count += 1;
                agg.total_size += size as u64;
                true
            }
        }
    }

    /// Build a retention tree for a single target class. Each tree node is a
    /// *class*; expanding a node walks one step toward the roots, grouping
    /// referencers by class.
    fn build_retention_tree(
        &self,
        graph: &HeapGraph,
        target_class: ClassID,
        max_depth: usize,
        retained_count_threshold: u64,
        retained_bytes_threshold: u64,
    ) -> TreeNode<ClassID, NodeAgg> {
        let mut root = TreeNode::new(target_class);

        // Collect every reachable instance of the target class.
        let mut instances: FastSet<ObjectID> = FastSet::default();
        let mut self_size: u64 = 0;
        for (&id, info) in graph.objects.iter() {
            if info.class_id == target_class {
                instances.insert(id);
                self_size += info.size as u64;
            }
        }
        let self_count = instances.len() as u64;
        root.value = Some(NodeAgg {
            instances,
            self_count,
            self_size,
        });

        // Track the chain of classes we are walking through, to short-circuit
        // tight retention cycles (e.g. doubly-linked lists where A→B→A).
        let mut on_path: FastSet<ClassID> = FastSet::default();
        on_path.insert(target_class);
        Self::expand_parents(
            graph,
            &mut root,
            0,
            max_depth,
            retained_count_threshold,
            retained_bytes_threshold,
            &mut on_path,
        );

        root
    }

    fn expand_parents(
        graph: &HeapGraph,
        node: &mut TreeNode<ClassID, NodeAgg>,
        depth: usize,
        max_depth: usize,
        retained_count_threshold: u64,
        retained_bytes_threshold: u64,
        on_path: &mut FastSet<ClassID>,
    ) {
        if depth >= max_depth {
            return;
        }

        // Group referencers of the current instances by their class. Scope the
        // immutable borrow on node.value so we can mutate node.children below.
        let groups: Vec<(ClassID, FastSet<ObjectID>)> = {
            let instances = match node.value.as_ref() {
                Some(v) => &v.instances,
                None => return,
            };
            let mut groups: FastMap<ClassID, FastSet<ObjectID>> = FastMap::default();
            for &inst in instances.iter() {
                if let Some(info) = graph.objects.get(&inst) {
                    for parent in &info.parents {
                        if let Some(pinfo) = graph.objects.get(&parent) {
                            if on_path.contains(&pinfo.class_id) {
                                continue;
                            }
                            groups.entry(pinfo.class_id).or_default().insert(parent);
                        }
                    }
                }
            }
            groups.into_iter().collect()
        };

        for (parent_class, parent_set) in groups {
            let count = parent_set.len() as u64;
            let size: u64 = parent_set
                .iter()
                .filter_map(|id| graph.objects.get(id).map(|i| i.size as u64))
                .sum();
            if count < retained_count_threshold || size < retained_bytes_threshold {
                continue;
            }

            let mut child = TreeNode::new(parent_class);
            child.value = Some(NodeAgg {
                instances: parent_set,
                self_count: count,
                self_size: size,
            });

            on_path.insert(parent_class);
            Self::expand_parents(
                graph,
                &mut child,
                depth + 1,
                max_depth,
                retained_count_threshold,
                retained_bytes_threshold,
                on_path,
            );
            on_path.remove(&parent_class);

            node.children.push(child);
        }
    }

    fn sort_tree(&self, tree: &mut TreeNode<ClassID, NodeAgg>) {
        let sort_by_size = self.session_info().get_parameter::<bool>("sort_by_size").unwrap();
        let compare = move |a: &TreeNode<ClassID, NodeAgg>, b: &TreeNode<ClassID, NodeAgg>| {
            let av = a.value.as_ref();
            let bv = b.value.as_ref();
            let (ak, bk) = if sort_by_size {
                (av.map_or(0, |v| v.self_size), bv.map_or(0, |v| v.self_size))
            } else {
                (av.map_or(0, |v| v.self_count), bv.map_or(0, |v| v.self_count))
            };
            bk.cmp(&ak) // descending
        };
        tree.sort_by_iterative(&compare);
    }

    fn build_and_report(&mut self) -> Result<(), HRESULT> {
        let graph = self.build_heap_graph();

        let max_depth = self.session_info().get_parameter::<usize>("max_depth").unwrap();
        let retained_count_threshold = self
            .session_info()
            .get_parameter::<usize>("retained_references_threshold")
            .unwrap() as u64;
        let retained_bytes_threshold = self
            .session_info()
            .get_parameter::<usize>("retained_bytes_threshold")
            .unwrap() as u64;
        let sort_by_size = self.session_info().get_parameter::<bool>("sort_by_size").unwrap();

        // Top-level classes: keep only those large enough to be worth showing,
        // sort by aggregate count or size.
        let mut candidates: Vec<(ClassID, ClassTotals)> = graph
            .class_totals
            .iter()
            .filter(|(_, c)| c.count >= retained_count_threshold && c.total_size >= retained_bytes_threshold)
            .map(|(&k, &v)| (k, v))
            .collect();
        candidates.sort_by(|a, b| {
            if sort_by_size {
                b.1.total_size.cmp(&a.1.total_size)
            } else {
                b.1.count.cmp(&a.1.count)
            }
        });

        let build_started = std::time::Instant::now();
        let mut trees: Vec<TreeNode<ClassID, NodeAgg>> = Vec::with_capacity(candidates.len());
        for (class_id, _) in candidates.iter() {
            let mut tree = self.build_retention_tree(
                &graph,
                *class_id,
                max_depth,
                retained_count_threshold,
                retained_bytes_threshold,
            );
            self.sort_tree(&mut tree);
            trees.push(tree);
        }
        info!(
            "Retention trees built for {} classes in {} ms",
            trees.len(),
            build_started.elapsed().as_millis()
        );

        self.write_report(&graph, trees)
    }

    fn write_report(
        &mut self,
        graph: &HeapGraph,
        trees: Vec<TreeNode<ClassID, NodeAgg>>,
    ) -> Result<(), HRESULT> {
        let started_at = std::time::Instant::now();
        let mut report = self.session_info.create_report("summary.html".to_owned());

        let total_objects: u64 = graph.class_totals.values().map(|c| c.count).sum();
        let total_bytes: u64 = graph.class_totals.values().map(|c| c.total_size).sum();

        report.write_line("<h2>GC Survivors Report</h2>".to_owned());
        report.write_line("This report lists objects that survived the last forced garbage collection. The top-level entries are <em>classes</em>; expand a class to walk one step toward the GC roots (i.e. up the retention chain). Each level groups referencers by class.".to_owned());

        report.write_line("<h4>Legend</h4>".to_owned());
        report.write_line(
            "<details open><summary> \
                <code>SomeClass</code> \
                <div class=\"chip\"><span>instances on this retention path / their total bytes</span><i class=\"material-icons\">radio_button_unchecked</i></div> \
                <div class=\"chip\"><span>handle</span><i class=\"material-icons\">flag</i></div> \
                <div class=\"chip\"><span>stack</span><i class=\"material-icons\">segment</i></div> \
                <div class=\"chip\"><span>finalizer</span><i class=\"material-icons\">auto_delete</i></div> \
                <div class=\"chip\"><span>other</span><i class=\"material-icons\">help</i></div> \
                </summary></details>"
                .to_owned(),
        );

        report.write_line(format!(
            "<h3>Survivors: {} objects / {} B across {} classes</h3>",
            (total_objects as usize).separate_by_policy(SEPARATOR_POLICY),
            (total_bytes as usize).separate_by_policy(SEPARATOR_POLICY),
            graph.class_totals.len().separate_by_policy(SEPARATOR_POLICY),
        ));

        for tree in trees.iter() {
            self.print_html(tree, &mut report);
        }

        info!("Report written in {} ms", started_at.elapsed().as_millis());
        Ok(())
    }

    fn print_html(&self, node: &TreeNode<ClassID, NodeAgg>, report: &mut Report) {
        let mut class_name = self.name_resolver.get_class_name(node.key);
        let escaped = html_escape::encode_text(&mut class_name);

        let (self_count, self_size, instances) = match &node.value {
            Some(v) => (v.self_count, v.self_size, Some(&v.instances)),
            None => (0, 0, None),
        };

        let mut line = format!(
            "<code>{escaped}</code> \
             <div class=\"chip\"><span>{} / {} B</span><i class=\"material-icons\">radio_button_unchecked</i></div>",
            (self_count as usize).separate_by_policy(SEPARATOR_POLICY),
            (self_size as usize).separate_by_policy(SEPARATOR_POLICY),
        );

        // If any instances at this node are themselves GC roots, surface the
        // root kinds. This naturally appears at the top of every retention
        // chain once we've walked far enough up to hit roots.
        if let Some(instances) = instances {
            let mut count_per_kind: HashMap<COR_PRF_GC_ROOT_KIND, u64> = HashMap::new();
            for id in instances.iter() {
                if let Some(kind) = self.root_kinds.get(id) {
                    *count_per_kind.entry(*kind).or_insert(0) += 1;
                }
            }
            for (kind, count) in count_per_kind {
                let icon = match kind {
                    COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_STACK => "segment",
                    COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_FINALIZER => "auto_delete",
                    COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_HANDLE => "flag",
                    COR_PRF_GC_ROOT_KIND::COR_PRF_GC_ROOT_OTHER => "help",
                };
                line.push_str(&format!(
                    "<div class=\"chip\"><span>{count}</span><i class=\"material-icons\">{icon}</i></div>"
                ));
            }
        }

        if node.children.is_empty() {
            report.write_line(format!("<li>{line}</li>"));
        } else {
            report.write_line(format!("<details><summary>{line}</summary>"));
            report.write_line("<ul>".to_owned());
            for child in &node.children {
                self.print_html(child, report);
            }
            report.write_line("</ul>".to_owned());
            report.write_line("</details>".to_owned());
        }
    }
}

impl CorProfilerCallback for GCSurvivorsProfiler {}

impl CorProfilerCallback2 for GCSurvivorsProfiler {
    fn garbage_collection_started(
        &mut self,
        generation_collected: &[ffi::BOOL],
        reason: ffi::COR_PRF_GC_REASON,
    ) -> Result<(), HRESULT> {
        let gen = ClrProfilerInfo::get_gc_gen(&generation_collected);
        info!("garbage_collection_started on gen {} for reason {:?}", gen, reason);
        if reason == ffi::COR_PRF_GC_REASON::COR_PRF_GC_INDUCED {
            self.is_relevant_gc.store(true, Ordering::Relaxed);
        }
        Ok(())
    }

    fn garbage_collection_finished(&mut self) -> Result<(), HRESULT> {
        info!("garbage_collection_finished");
        if !self.is_relevant_gc.load(Ordering::Relaxed) {
            error!("Early return: GC was not the one we induced");
            return Ok(());
        }

        let _ = self.build_and_report();

        let profiler_info = self.clr().clone();
        profiler_info.request_profiler_detach(3000).ok();
        Ok(())
    }

    fn root_references_2(
        &mut self,
        root_ref_ids: &[ObjectID],
        root_kinds: &[COR_PRF_GC_ROOT_KIND],
        _root_flags: &[COR_PRF_GC_ROOT_FLAGS],
        _root_ids: &[UINT_PTR],
    ) -> Result<(), HRESULT> {
        if !self.is_relevant_gc.load(Ordering::Relaxed) {
            warn!("Early return: GC was not the one we induced");
            return Ok(());
        }
        for i in 0..root_ref_ids.len() {
            let root_id = root_ref_ids[i];
            if root_id == 0 {
                continue;
            }
            self.roots.push(root_id);
            self.root_kinds.entry(root_id).or_insert(root_kinds[i]);
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
        self.init(
            ffi::COR_PRF_MONITOR::COR_PRF_MONITOR_GC,
            None,
            profiler_info,
            client_data,
            client_data_length,
        )
    }

    fn profiler_attach_complete(&mut self) -> Result<(), HRESULT> {
        self.name_resolver = CachedNameResolver::new(self.clr().clone());

        // ForceGC must run on a thread without profiler callbacks on its stack.
        let clr = self.clr().clone();
        let _ = thread::spawn(move || match clr.force_gc() {
            Ok(_) => debug!("GC forced"),
            Err(hresult) => error!("Error forcing GC: {:?}", hresult),
        })
        .join();

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
