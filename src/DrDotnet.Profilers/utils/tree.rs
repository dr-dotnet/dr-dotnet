use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Clone)]
struct TreeNode<K, D> {
    pub key: K,
    pub value: Option<D>,
    pub children: Vec<TreeNode<K, D>>,
}

impl<K, D> TreeNode<K, D>
    where K: PartialEq + Copy, D: Clone
{
    pub fn new(key: K, value: Option<D>) -> Self {
        TreeNode {
            key,
            value,
            children: Vec::new(),
        }
    }

    // Sort children recursively based on the given closure
    pub fn sort_by<F>(&mut self, compare: &F)
        where F: Fn(&TreeNode<K, D>, &TreeNode<K, D>) -> Ordering,
    {
        self.children.sort_by(compare);
        for child in &mut self.children {
            child.sort_by(compare);
        }
    }

    pub fn build_from_sequences(sequences: &HashMap<Vec<K>, D>, root_key: K) -> TreeNode<K, D> {
        let mut root = TreeNode::new(root_key, None);
        for (sequence, value) in sequences {
            let mut current = &mut root;
            let mut depth: usize = 1;
            let l = sequence.len();
            for y in sequence {
                let mut child = if let Some(i) = current.children.iter().position(|child| child.key.eq(&y)) {
                    &mut current.children[i]
                } else {
                    let mut new_child: TreeNode<K, D> = TreeNode::new(*y, if depth == l { Some(value.clone()) } else { None });
                    current.children.push(new_child);
                    let len = current.children.len();
                    &mut current.children[len - 1]
                };
                current = child;
                depth+=1;
            }
        }
        return root;
    }
}

impl<K, D> PartialEq for TreeNode<K, D>
    where K: Eq
{
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key || self.children.len() != other.children.len() {
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

impl<K, D> Eq for TreeNode<K, D>
    where K: Eq {}

impl<K, D> TreeNode<K, D>
    where D: AddAssign + Default + Clone
{
    // Compute recursively and return the inclusive value of a given TreeNode
    pub fn get_inclusive_value(&self) -> D
    {
        let mut inclusive_data = if let Some(self_data) = &self.value {
            self_data.clone()
        } else {
            D::default()
        };
        for child in self.children.iter() {
            let child_inclusive_value = child.get_inclusive_value();
            inclusive_data += child_inclusive_value;
        }
        inclusive_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print<T, D, F>(tree: &TreeNode<T, D>, depth: usize, format: &F)
        where F: Fn(&TreeNode<T, D>) -> String
    {
        let tabs = " ".repeat(depth);
        println!("{}- {}", tabs, format(tree));

        for child in &tree.children {
            print(child, depth + 1, format);
        }
    }

    // Run tests with 'cargo test -- --nocapture' to get output in console
    #[test]
    fn test_tree() {
        // Sequences of u32
        let sequences: HashMap<Vec<u32>, usize> = HashMap::from([
            (vec![1, 2, 3], 1),
            (vec![2, 2, 3], 2),
            (vec![1, 2], 3),
            (vec![1, 2, 4], 5),
            (vec![1, 3, 5], 6),
            (vec![2, 3, 2, 1, 4], 7),
            (vec![1, 3, 5, 1], 8),
        ]);

        // Expected sequences in a tree, sorted by descending inclusive value
        let expected =
        TreeNode { key: 0, value: None, children: vec![
            TreeNode { key: 1, value: None, children: vec![
                TreeNode { key: 3, value: None, children: vec![
                    TreeNode { key: 5, value: Some(6), children: vec![
                        TreeNode { key: 1, value: Some(8), children: vec![] }] }] },
                TreeNode { key: 2, value: Some(3), children: vec![
                    TreeNode { key: 4, value: Some(5), children: vec![] },
                    TreeNode { key: 3, value: Some(1), children: vec![] }] }] },
            TreeNode { key: 2, value: None, children: vec![
                TreeNode { key: 3, value: None, children: vec![
                    TreeNode { key: 2, value: None, children: vec![
                        TreeNode { key: 1, value: None, children: vec![
                            TreeNode { key: 4, value: Some(7), children: vec![] }] }] }] },
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
        impl AddAssign for Threads {
            fn add_assign(&mut self, other: Self) {
                self.0.extend(other.0);
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