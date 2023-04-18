use std::{ops::AddAssign, collections::HashMap, cell::RefCell};
use profilers::utils::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

type FunctionID = u32;
type ThreadID = u32;

// Required to wrap Vec<ThreadID> in order to implement AddAssign
#[derive(Clone, Default, Debug, Eq)]
pub struct Threads(Vec<ThreadID>);

impl PartialEq<Self> for Threads {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// Implement AddAssign for get_inclusive_value to be usable
impl AddAssign<&Threads> for Threads {
    fn add_assign(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }
}

fn build_random_sequences() -> HashMap<Vec<FunctionID>, Threads> {
    let mut r = StdRng::seed_from_u64(222);

    let mut sequences: HashMap<Vec<FunctionID>, Threads> = HashMap::new();

    // Build some random sequences
    for i in 0..100000 {
        let mut func_ids: Vec<FunctionID> = Vec::new();
        for j in 0..r.gen_range(3..10) {
            func_ids.push(r.gen_range(1..100));
        }
        let mut thread_ids: Vec<ThreadID> = Vec::new();
        for j in 0..r.gen_range(3..10) {
            thread_ids.push(r.gen_range(1..100));
        }
        sequences.insert(func_ids, Threads(thread_ids));
    }

    return sequences;
}

fn bench_tree_sort(c: &mut Criterion) {

    let sequences = build_random_sequences();

    c.bench_function("recursive", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        b.iter(|| tree.sort_by(&|a, b| b.inclusive_value.0.len().cmp(&a.inclusive_value.0.len())))
    });
    
    c.bench_function("iterative", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        b.iter(|| tree.sort_by_iterative(&|a, b| b.inclusive_value.0.len().cmp(&a.inclusive_value.0.len())))
    });

    c.bench_function("iterative calculate", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        b.iter(|| tree.sort_by_iterative(&|a, b| b.calculate_inclusive_value().0.len().cmp(&a.calculate_inclusive_value().0.len())))
    });
    
    c.bench_function("iterative cached", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        let cache: RefCell<HashMap<u32, usize>> = RefCell::new(HashMap::new());
        b.iter(|| {
            tree.sort_by_iterative(&|a, b| {
                let mut c = cache.borrow_mut();
        
                let value_b = *c
                    .entry(b.key)
                    .or_insert_with(|| b.calculate_inclusive_value().0.len());

                let value_a = *c
                    .entry(a.key)
                    .or_insert_with(|| a.calculate_inclusive_value().0.len());

                value_b.cmp(&value_a)
            })
        });
    });

    c.bench_function("multithreaded", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        b.iter(|| tree.sort_by_multithreaded(&|a, b| b.inclusive_value.0.len().cmp(&a.inclusive_value.0.len())))
    });
}

fn bench_tree_build(c: &mut Criterion) {

    let sequences = build_random_sequences();

    c.bench_function("build tree", |b| {
        b.iter(|| TreeNode::build_from_sequences(&sequences, 0))
    });
}

criterion_group!(benches, bench_tree_sort, bench_tree_build);
criterion_main!(benches);