use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode<K, V> {
    pub key: K,
    pub value: Option<V>,
    pub children: Vec<TreeNode<K, V>>,
}

impl<K, V> TreeNode<K, V>
where
    K: PartialEq + Eq + Copy,
    V: Clone,
{
    pub fn new(key: K) -> Self {
        TreeNode {
            key,
            value: None,
            children: Vec::new(),
        }
    }

    /// Sort children at every node, depth-first. Iterative so we never blow
    /// the stack on deep trees.
    ///
    /// `compare` is invoked O(n log n) times *per parent*. If the closure
    /// itself is expensive (e.g. calls `compute_inclusive_value`), memoize on
    /// the caller side — see the bench for an example.
    pub fn sort_by<F>(&mut self, compare: &F)
    where
        F: Fn(&TreeNode<K, V>, &TreeNode<K, V>) -> Ordering,
    {
        let mut stack: Vec<&mut TreeNode<K, V>> = vec![self];
        while let Some(node) = stack.pop() {
            node.children.sort_by(compare);
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
                    current.children.push(TreeNode::new(*y));
                    let len = current.children.len();
                    &mut current.children[len - 1]
                };
                current = child;
            }
            current.value = Some(value.clone());
        }
        root
    }
}

impl<K, V> TreeNode<K, V>
where
    V: for<'a> AddAssign<&'a V> + Default,
{
    fn accumulate_inclusive(&self, value: &mut V) {
        if let Some(self_data) = &self.value {
            value.add_assign(self_data);
        }
        for child in self.children.iter() {
            child.accumulate_inclusive(value);
        }
    }

    /// Walk the whole subtree and accumulate `value` over every node via
    /// `AddAssign`. Cost is O(subtree); cache the result if you intend to use
    /// it during a sort comparator.
    pub fn compute_inclusive_value(&self) -> V {
        let mut value = V::default();
        self.accumulate_inclusive(&mut value);
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
        let sequences: HashMap<Vec<u32>, usize> = HashMap::from([
            (vec![1, 2, 3], 1),
            (vec![2, 2, 3], 2),
            (vec![1, 2], 3),
            (vec![1, 2, 4], 4),
            (vec![1, 3, 5], 5),
            (vec![2, 3, 2, 1, 4], 6),
            (vec![1, 3, 5, 1], 7),
        ]);

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
            format!("{} [inc:{}, exc:{:?}]", node.key, node.compute_inclusive_value(), node.value)
        });
        assert_ne!(tree, expected);

        let mut tree_clone = tree.clone();
        assert_ne!(tree_clone, expected);
        let start = Instant::now();
        tree_clone.sort_by(&|a, b| b.compute_inclusive_value().cmp(&a.compute_inclusive_value()));
        let duration = start.elapsed();
        println!("sort_by duration: {:?}", duration);
        assert_eq!(tree_clone, expected);

        print(&tree_clone, 0, &|node: &TreeNode<u32, usize>| {
            format!("{} [inc:{}, exc:{:?}]", node.key, node.compute_inclusive_value(), node.value)
        });
    }
}
