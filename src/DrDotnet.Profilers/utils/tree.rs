use std::cmp::Ordering;
use std::collections::HashMap;
use core::fmt::Display;

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    exclusive: usize,
    inclusive: usize,
    depth: usize, // Do we need this?
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T>
    where T: PartialEq + Copy
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
            exclusive: 0,
            inclusive: 0,
            depth: 0,
        }
    }

    fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }

    // Sort children recursively based on the given closure
    fn sort_by<F>(&mut self, compare: &F)
        where F: Fn(&TreeNode<T>, &TreeNode<T>) -> Ordering,
    {
        self.children.sort_by(compare);
        for child in &mut self.children {
            child.sort_by(compare);
        }
    }

    fn build_from_sequences(sequences: &HashMap<Vec<T>, usize>, root_value: T) -> TreeNode<T> {
        let mut root = TreeNode::new(root_value);
        for (sequence, value) in sequences {
            root.inclusive += value;
            let mut current = &mut root;
            let mut depth: usize = 1;
            let l = sequence.len();
            for y in sequence {
                let mut child = if let Some(i) = current.children.iter().position(|child| child.value.eq(&y)) {
                    &mut current.children[i]
                } else {
                    let mut new_child: TreeNode<T> = TreeNode::new(*y);
                    new_child.exclusive = if depth == l { *value } else { 0 };
                    current.children.push(new_child);
                    let len = current.children.len();
                    &mut current.children[len - 1]
                };
                child.depth = depth;
                child.inclusive += value;
                current = child;
                depth+=1;
            }
        }
        return root;
    }
}

// Implement the PartialEq trait for TreeNode
impl<T> PartialEq for TreeNode<T>
    where T: Eq
{
    fn eq(&self, other: &Self) -> bool {
        if self.value != other.value || self.children.len() != other.children.len() {
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

// Implement the Eq trait for TreeNode
impl<T> Eq for TreeNode<T>
    where T: Eq {}

#[cfg(test)]
mod tests {
    use super::*;

    fn print<T>(tree: &TreeNode<T>, depth: usize)
        where T: Display
    {
        let tabs = " ".repeat(depth);
        println!("{}- {} (inc:{}, exc:{})", tabs, tree.value, tree.inclusive, tree.exclusive);

        for child in &tree.children {
            print(child, depth + 1);
        }
    }

    // Run tests with 'cargo test -- --nocapture' to get output in console
    #[test]
    fn test_print() {
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

        let tree = TreeNode::build_from_sequences(&sequences, 0);

        print(&tree, 0);
    }

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
        TreeNode { value: 0, exclusive: 0, inclusive: 32, depth: 0, children: vec![
            TreeNode { value: 1, exclusive: 0, inclusive: 23, depth: 1, children: vec![
                TreeNode { value: 3, exclusive: 0, inclusive: 14, depth: 2, children: vec![
                    TreeNode { value: 5, exclusive: 0, inclusive: 14, depth: 3, children: vec![
                        TreeNode { value: 1, exclusive: 8, inclusive: 8, depth: 4, children: vec![] }] }] },
                TreeNode { value: 2, exclusive: 3, inclusive: 9, depth: 2, children: vec![
                    TreeNode { value: 4, exclusive: 5, inclusive: 5, depth: 3, children: vec![] },
                    TreeNode { value: 3, exclusive: 1, inclusive: 1, depth: 3, children: vec![] }] }] },
            TreeNode { value: 2, exclusive: 0, inclusive: 9, depth: 1, children: vec![
                TreeNode { value: 3, exclusive: 0, inclusive: 7, depth: 2, children: vec![
                    TreeNode { value: 2, exclusive: 0, inclusive: 7, depth: 3, children: vec![
                        TreeNode { value: 1, exclusive: 0, inclusive: 7, depth: 4, children: vec![
                            TreeNode { value: 4, exclusive: 7, inclusive: 7, depth: 5, children: vec![] }] }] }] },
                TreeNode { value: 2, exclusive: 0, inclusive: 2, depth: 2, children: vec![
                    TreeNode { value: 3, exclusive: 2, inclusive: 2, depth: 3, children: vec![] }] }] }] };

        let mut tree = TreeNode::build_from_sequences(&sequences, 0);

        assert_ne!(tree, expected);

        // Sorts by descending inclusive value
        tree.sort_by(&|a, b| b.inclusive.cmp(&a.inclusive));

        assert_eq!(tree, expected);
    }
}