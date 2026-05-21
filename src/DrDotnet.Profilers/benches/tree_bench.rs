use criterion::{criterion_group, criterion_main, Criterion};
use profilers::utils::*;
use rand::prelude::*;
use std::{cell::RefCell, collections::HashMap, ops::AddAssign};

type FunctionID = u32;
type ThreadID = u32;

// Wrapper around Vec<ThreadID> so we can implement AddAssign for it.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Threads(Vec<ThreadID>);

// Required by compute_inclusive_value.
impl AddAssign<&Threads> for Threads {
    fn add_assign(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }
}

fn build_random_sequences() -> HashMap<Vec<FunctionID>, Threads> {
    let mut r = StdRng::seed_from_u64(222);

    let mut sequences: HashMap<Vec<FunctionID>, Threads> = HashMap::new();

    for _ in 0..100000 {
        let mut func_ids: Vec<FunctionID> = Vec::new();
        for _ in 0..r.gen_range(3..10) {
            func_ids.push(r.gen_range(1..100));
        }
        let mut thread_ids: Vec<ThreadID> = Vec::new();
        for _ in 0..r.gen_range(3..10) {
            thread_ids.push(r.gen_range(1..100));
        }
        sequences.insert(func_ids, Threads(thread_ids));
    }

    sequences
}

fn bench_tree_sort(c: &mut Criterion) {
    let sequences = build_random_sequences();

    c.bench_function("sort_by + no caching", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        b.iter(|| {
            tree.sort_by(&|a, b| {
                b.compute_inclusive_value()
                    .0
                    .len()
                    .cmp(&a.compute_inclusive_value().0.len())
            })
        })
    });

    c.bench_function("sort_by + caching", |b| {
        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        let cache: RefCell<HashMap<u32, usize>> = RefCell::new(HashMap::new());
        b.iter(|| {
            tree.sort_by(&|a, b| {
                let mut c = cache.borrow_mut();
                let value_b = *c.entry(b.key).or_insert_with(|| b.compute_inclusive_value().0.len());
                let value_a = *c.entry(a.key).or_insert_with(|| a.compute_inclusive_value().0.len());
                value_b.cmp(&value_a)
            })
        });
    });
}

fn bench_tree_build(c: &mut Criterion) {
    let sequences = build_random_sequences();

    c.bench_function("build tree", |b| b.iter(|| TreeNode::build_from_sequences(&sequences, 0)));
}

criterion_group!(benches, bench_tree_sort, bench_tree_build);
criterion_main!(benches);
