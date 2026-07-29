use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::ops::AddAssign;

#[derive(Debug, Clone)]
pub struct TreeNode<K, V> {
    pub key: K,
    pub value: Option<V>,
    pub children: Vec<TreeNode<K, V>>,
}

impl<K, V> TreeNode<K, V>
where
    K: PartialEq + Eq + Copy + Sync + Send,
    V: Clone + Sync + Send,
{
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            value: None,
            children: Vec::new(),
        }
    }

    pub fn log<F>(&self, depth: usize, format: &F)
    where
        F: Fn(&Self) -> String,
    {
        let tabs = " ".repeat(depth);
        info!("{}- {}", tabs, format(self));

        for child in self.children.iter() {
            child.log::<F>(depth + 1, format);
        }
    }

    // Sort children recursively based on the given closure
    pub fn sort_by<F>(&mut self, compare: &F)
    where
        F: Fn(&TreeNode<K, V>, &TreeNode<K, V>) -> Ordering,
    {
        self.children.sort_by(compare);
        for child in &mut self.children {
            child.sort_by(compare);
        }
    }

    pub fn sort_by_iterative<F>(&mut self, compare: &F)
    where
        F: Fn(&TreeNode<K, V>, &TreeNode<K, V>) -> Ordering,
    {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while let Some(node) = queue.pop_front() {
            node.children.sort_by(compare);
            for child in &mut node.children {
                queue.push_back(child);
            }
        }
    }

    pub fn sort_by_multithreaded<F>(&mut self, compare: &F)
    where
        F: Fn(&TreeNode<K, V>, &TreeNode<K, V>) -> Ordering + Sync,
    {
        let mut stack = vec![self];
        while let Some(node) = stack.pop() {
            node.children.par_sort_by(compare);
            for child in &mut node.children {
                stack.push(child);
            }
        }
    }

    pub fn add_sequence<I>(&mut self, sequence: I) -> &mut TreeNode<K, V>
    where
        I: IntoIterator<Item = K>,
    {
        let mut current_node = self;
        for element in sequence {
            if let Some(i) = current_node.children.iter().position(|x| x.key == element) {
                current_node = &mut current_node.children[i];
            } else {
                current_node.children.push(TreeNode::new(element));
                let new_index = current_node.children.len() - 1;
                current_node = &mut current_node.children[new_index];
            }
        }
        current_node
    }

    pub fn build_from_sequences(sequences: &HashMap<Vec<K>, V>, root_key: K) -> TreeNode<K, V> {
        let mut root = TreeNode::new(root_key);

        for (sequence, value) in sequences {
            let mut current = &mut root;
            for y in sequence {
                let child = if let Some(i) = current.children.iter().position(|child| child.key.eq(&y)) {
                    &mut current.children[i]
                } else {
                    let new_child: TreeNode<K, V> = TreeNode::new(*y);
                    current.children.push(new_child);
                    let len = current.children.len();
                    &mut current.children[len - 1]
                };
                current = child;
            }
            current.value = Some(value.clone());
        }
        return root;
    }
}

impl<K, V> TreeNode<K, V>
where
    K: Copy,
    V: Default + for<'a> AddAssign<&'a V>,
{
    // Number of sequences held by this tree, ie. the number of nodes carrying a value
    pub fn count_sequences(&self) -> usize {
        self.children.iter().map(Self::count_sequences).sum::<usize>() + usize::from(self.value.is_some())
    }

    /// Keeps only the `max_children` first children of every node, recursively.
    ///
    /// Children are expected to be sorted beforehand, since the first ones are the ones kept. Nodes
    /// that lost children get one extra child keyed with `placeholder_key`, holding the inclusive
    /// value of everything that was dropped, so that inclusive values remain exact. Such a node
    /// therefore ends up with `max_children` + 1 children.
    ///
    /// Returns the number of dropped sequences.
    pub fn keep_top_children(&mut self, max_children: usize, placeholder_key: K) -> usize {
        let mut dropped_sequences = 0;

        if self.children.len() > max_children {
            let mut dropped_value = V::default();
            for dropped_child in self.children.drain(max_children..) {
                dropped_sequences += dropped_child.count_sequences();
                dropped_value.add_assign(&dropped_child.get_inclusive_value());
            }
            self.children.push(TreeNode {
                key: placeholder_key,
                value: Some(dropped_value),
                children: Vec::new(),
            });
        }

        for child in &mut self.children {
            dropped_sequences += child.keep_top_children(max_children, placeholder_key);
        }

        dropped_sequences
    }
}

impl<K, V> PartialEq for TreeNode<K, V>
where
    K: Eq,
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key || self.value != other.value || self.children.len() != other.children.len() {
            return false;
        }
        for (child1, child2) in self.children.iter().zip(other.children.iter()) {
            if child1 != child2 {
                return false;
            }
        }
        true
    }
}

impl<K, V> Eq for TreeNode<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> TreeNode<K, V>
where
    V: for<'a> AddAssign<&'a V> + Default,
{
    fn get_inclusive_value_recursive(&self, value: &mut V) {
        if let Some(self_data) = &self.value {
            value.add_assign(self_data);
        }
        for child in self.children.iter() {
            child.get_inclusive_value_recursive(value);
        }
    }

    // Compute recursively and return the inclusive value of a given TreeNode
    pub fn get_inclusive_value(&self) -> V {
        // Creating a single vector and passing it through get_inclusive_value_recursive
        // enables us to avoid cloning.
        let mut value = V::default();
        self.get_inclusive_value_recursive(&mut value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn print<T, V, F>(tree: &TreeNode<T, V>, depth: usize, format: &F)
    where
        F: Fn(&TreeNode<T, V>) -> String,
    {
        let tabs = " ".repeat(depth);
        println!("{}- {}", tabs, format(tree));

        for child in &tree.children {
            print(child, depth + 1, format);
        }
    }

    // Run tests with 'cargo test -- --nocapture --test-threads=1' to get output in console
    #[test]
    fn test_tree() {
        // Sequences of u32
        let sequences: HashMap<Vec<u32>, usize> = HashMap::from([
            (vec![1, 2, 3], 1),
            (vec![2, 2, 3], 2),
            (vec![1, 2], 3),
            (vec![1, 2, 4], 4),
            (vec![1, 3, 5], 5),
            (vec![2, 3, 2, 1, 4], 6),
            (vec![1, 3, 5, 1], 7),
        ]);

        // Expected sequences in a tree, sorted by descending inclusive value
        let expected = TreeNode {
            key: 0,
            value: None,
            children: vec![
                TreeNode {
                    key: 1,
                    value: None,
                    children: vec![
                        TreeNode {
                            key: 3,
                            value: None,
                            children: vec![TreeNode {
                                key: 5,
                                value: Some(5),
                                children: vec![TreeNode {
                                    key: 1,
                                    value: Some(7),
                                    children: vec![],
                                }],
                            }],
                        },
                        TreeNode {
                            key: 2,
                            value: Some(3),
                            children: vec![
                                TreeNode {
                                    key: 4,
                                    value: Some(4),
                                    children: vec![],
                                },
                                TreeNode {
                                    key: 3,
                                    value: Some(1),
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                },
                TreeNode {
                    key: 2,
                    value: None,
                    children: vec![
                        TreeNode {
                            key: 3,
                            value: None,
                            children: vec![TreeNode {
                                key: 2,
                                value: None,
                                children: vec![TreeNode {
                                    key: 1,
                                    value: None,
                                    children: vec![TreeNode {
                                        key: 4,
                                        value: Some(6),
                                        children: vec![],
                                    }],
                                }],
                            }],
                        },
                        TreeNode {
                            key: 2,
                            value: None,
                            children: vec![TreeNode {
                                key: 3,
                                value: Some(2),
                                children: vec![],
                            }],
                        },
                    ],
                },
            ],
        };

        let tree = TreeNode::build_from_sequences(&sequences, 0);

        println!("Unsorted:");
        print(&tree, 0, &|node: &TreeNode<u32, usize>| {
            format!("{} [inc:{}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value)
        });
        assert_ne!(tree, expected);

        // Sorts by descending inclusive value
        let mut tree_clone = tree.clone();
        assert_ne!(tree_clone, expected);
        let start = Instant::now();
        tree_clone.sort_by(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));
        let duration = start.elapsed();
        println!("Recursive sort_by duration: {:?}", duration);
        assert_eq!(tree_clone, expected);

        let mut tree_clone = tree.clone();
        assert_ne!(tree_clone, expected);
        let start = Instant::now();
        tree_clone.sort_by_iterative(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));
        let duration = start.elapsed();
        println!("Iterative sort_by duration: {:?}", duration);
        assert_eq!(tree_clone, expected);

        let mut tree_clone = tree.clone();
        assert_ne!(tree_clone, expected);
        let start = Instant::now();
        tree_clone.sort_by_multithreaded(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));
        let duration = start.elapsed();
        println!("Multithreaded sort_by duration: {:?}", duration);
        assert_eq!(tree_clone, expected);

        print(&tree_clone, 0, &|node: &TreeNode<u32, usize>| {
            format!("{} [inc:{}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value)
        });
    }

    #[test]
    fn test_keep_top_children() {
        let sequences: HashMap<Vec<u32>, usize> = HashMap::from([
            (vec![1, 2, 3], 10),
            (vec![1, 2, 4], 20),
            (vec![1, 2], 3),
            (vec![5, 6], 7),
            (vec![5, 7], 1),
            (vec![8], 5),
        ]);

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);
        tree.sort_by(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));
        assert_eq!(tree.count_sequences(), 6);
        let total = tree.get_inclusive_value();
        assert_eq!(total, 46);

        // Roots are 1 (33), 5 (8) and 8 (5), so only 8 and its single sequence are dropped
        let mut tree_of_two = tree.clone();
        assert_eq!(tree_of_two.keep_top_children(2, u32::MAX), 1);

        print(&tree_of_two, 0, &|node: &TreeNode<u32, usize>| {
            format!("{} [inc:{}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value)
        });

        assert_eq!(
            tree_of_two,
            TreeNode {
                key: 0,
                value: None,
                children: vec![
                    TreeNode {
                        key: 1,
                        value: None,
                        children: vec![TreeNode {
                            key: 2,
                            value: Some(3),
                            children: vec![
                                TreeNode {
                                    key: 4,
                                    value: Some(20),
                                    children: vec![]
                                },
                                TreeNode {
                                    key: 3,
                                    value: Some(10),
                                    children: vec![]
                                },
                            ],
                        }],
                    },
                    TreeNode {
                        key: 5,
                        value: None,
                        children: vec![
                            TreeNode {
                                key: 6,
                                value: Some(7),
                                children: vec![]
                            },
                            TreeNode {
                                key: 7,
                                value: Some(1),
                                children: vec![]
                            },
                        ],
                    },
                    // Inclusive value of the dropped 8 root
                    TreeNode {
                        key: u32::MAX,
                        value: Some(5),
                        children: vec![]
                    },
                ],
            }
        );

        // Dropped values are moved to placeholder nodes, so the total is always preserved
        assert_eq!(tree_of_two.get_inclusive_value(), total);

        // Keeping a single child drops the 5 and 8 roots, then the 3 leaf under 2
        let mut tree_of_one = tree.clone();
        assert_eq!(tree_of_one.keep_top_children(1, u32::MAX), 4);
        assert_eq!(tree_of_one.get_inclusive_value(), total);
        assert_eq!(tree_of_one.children.last().unwrap().value, Some(13));

        // Keeping more children than any node has is a no-op
        let mut tree_untouched = tree.clone();
        assert_eq!(tree_untouched.keep_top_children(100, u32::MAX), 0);
        assert_eq!(tree_untouched, tree);
    }
}
