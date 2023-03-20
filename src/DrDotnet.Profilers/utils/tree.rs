use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Clone)]
struct TreeNode<K, V> {
    pub key: K,
    pub value: Option<V>,
    pub children: Vec<TreeNode<K, V>>,
}

impl<K, V> TreeNode<K, V>
    where K: PartialEq + Copy, V: Clone
{
    pub fn new(key: K) -> Self {
        TreeNode {
            key: key,
            value: None,
            children: Vec::new(),
        }
    }

    // Sort children recursively based on the given closure
    pub fn sort_by<F>(&mut self, compare: &F)
        where F: Fn(&TreeNode<K, V>, &TreeNode<K, V>) -> Ordering,
    {
        self.children.sort_by(compare);
        for child in &mut self.children {
            child.sort_by(compare);
        }
    }

    pub fn build_from_sequences(sequences: &HashMap<Vec<K>, V>, root_key: K) -> TreeNode<K, V> {
        let mut root = TreeNode::new(root_key);
        for (sequence, value) in sequences {
            let mut current = &mut root;
            for y in sequence {
                let mut child = if let Some(i) = current.children.iter().position(|child| child.key.eq(&y)) {
                    &mut current.children[i]
                } else {
                    let mut new_child: TreeNode<K, V> = TreeNode::new(*y);
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

impl<K, V> PartialEq for TreeNode<K, V>
    where K: Eq, V: Eq
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
    where K: Eq, V: Eq {}

impl<K, V> TreeNode<K, V>
    where V: for<'a> AddAssign<&'a V> + Default,
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

    fn print<T, V, F>(tree: &TreeNode<T, V>, depth: usize, format: &F)
        where F: Fn(&TreeNode<T, V>) -> String
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
        let expected =
        TreeNode { key: 0, value: None, children: vec![
            TreeNode { key: 1, value: None, children: vec![
                TreeNode { key: 3, value: None, children: vec![
                    TreeNode { key: 5, value: Some(5), children: vec![
                        TreeNode { key: 1, value: Some(7), children: vec![] }] }] },
                TreeNode { key: 2, value: Some(3), children: vec![
                    TreeNode { key: 4, value: Some(4), children: vec![] },
                    TreeNode { key: 3, value: Some(1), children: vec![] }] }] },
            TreeNode { key: 2, value: None, children: vec![
                TreeNode { key: 3, value: None, children: vec![
                    TreeNode { key: 2, value: None, children: vec![
                        TreeNode { key: 1, value: None, children: vec![
                            TreeNode { key: 4, value: Some(6), children: vec![] }] }] }] },
                TreeNode { key: 2, value: None, children: vec![
                    TreeNode { key: 3, value: Some(2), children: vec![] }] }] }] };

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);

        println!("Unsorted:");
        print(&tree, 0, &|node: &TreeNode<u32, usize>| format!("{} [inc:{}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value));

        assert_ne!(tree, expected);

        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.get_inclusive_value().cmp(&a.get_inclusive_value()));

        println!("Sorted:");
        print(&tree, 0, &|node: &TreeNode<u32, usize>| format!("{} [inc:{}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value));

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_pstack() {

        type FunctionID = u32;
        type ThreadID = u32;
    
        // Required to wrap Vec<ThreadID> in order to implement AddAssign
        #[derive(Clone, Default, Debug)]
        pub struct Threads(Vec<ThreadID>);
    
        // Implement AddAssign for get_inclusive_value to be usable
        impl AddAssign<&Threads> for Threads {
            fn add_assign(&mut self, other: &Self) {
                self.0.extend(&other.0);
            }
        }

        let sequences: HashMap<Vec<FunctionID>, Threads> = HashMap::from([
            (vec![1, 2, 3], Threads(vec![1001, 1002, 1003])),
            (vec![2, 2, 3], Threads(vec![1004, 1005])),
            (vec![1, 2], Threads(vec![1006])),
            (vec![1, 2, 4], Threads(vec![1007, 1008, 1009])),
            (vec![1, 3, 5], Threads(vec![1010])),
            (vec![2, 3, 2, 1, 4], Threads(vec![1010, 1011, 1012, 1013])),
            (vec![1, 3, 5, 1], Threads(vec![1014, 1015])),
        ]);

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);

        println!("Unsorted:");
        print(&tree, 0, &|node| format!("{} [inc:{:?}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value));

        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.get_inclusive_value().0.len().cmp(&a.get_inclusive_value().0.len()));

        println!("Sorted:");
        print(&tree, 0, &|node| format!("{} [inc:{:?}, exc:{:?}]", node.key, node.get_inclusive_value(), node.value));
    }
}