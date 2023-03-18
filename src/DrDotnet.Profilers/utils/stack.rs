use crate::api::ffi::HRESULT;

use std::hash::{ Hash, Hasher };

pub unsafe extern "system" fn stack_snapshot_callback(method_id: usize, ip: usize, frame_info: usize, context_size: u32, context: *const u8, client_data: *mut libc::c_void) -> HRESULT {
    let vec = &mut *client_data.cast::<Vec<usize>>();
    vec.push(method_id);
    return HRESULT::S_OK;
}

pub struct Stack {
    pub frames: Vec<u32>
}

impl PartialEq for Stack {
    fn eq(&self, other: &Self) -> bool {
        self.frames == other.frames
    }
}
impl Eq for Stack {}

impl Hash for Stack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for frame in &self.frames {
            frame.hash(state);
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeNode {
    pub exclusive: u32,
    pub inclusive: u32,
    pub data: u32,
    pub depth: u32,
    pub children: HashMap<u32, TreeNode>,
}

impl TreeNode {
    pub fn new_from_flat(flat: HashMap<Vec<u32>, u32>) -> Self {
        let mut root = TreeNode { depth: 0, children: HashMap::new(), exclusive: 0, inclusive: 0, data: 0 };
        for x in flat {
            root.inclusive += x.1;
            let mut current = &mut root.children;
            let mut i: usize = 1;
            let l = x.0.len();
            for y in x.0 {
                let k = current
                    .entry(y)
                    .and_modify(|node| node.inclusive += x.1)
                    .or_insert(TreeNode { depth: i as u32, data: y, children: HashMap::new(), inclusive: x.1, exclusive: if i == l { x.1 } else { 0 } });
                current = &mut k.children;
                i+=1;
            }
        }
        root
    }

    // pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&u32, &TreeNode)> + 'a> {
    //     Box::new(
    //         self.children
    //             .iter()
    //             .chain(self.children.iter().map(|n| n.1.iter()).flatten()),
    //     )
    // }

    // pub fn iter_sorted<'a>(&'a self) -> Box<dyn Iterator<Item = (&u32, &TreeNode)> + 'a> {
    //     let vec: Vec<_> = self.children.iter().collect();
    //     Box::new(
    //         vec.iter()
    //             .chain(self.children.iter().map(|n| n.1.iter_sorted()).flatten()),
    //     )
    // }
}

use std::collections::VecDeque;

struct Iter<'a> {
    stack: VecDeque<&'a TreeNode>
}

impl<'a> Iter<'a> {
    fn new(root: &'a TreeNode) -> Self {
        Self {
            stack: [root].into()
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u32, u32, u32); // return type of state_to_value
    fn next(&mut self) -> Option<Self::Item> {
        let state = self.stack.pop_back()?;
        use itertools::Itertools;
        //let vec: Vec<_> = state.children.values().collect();
        //self.stack.extend(vec);
        for s in state.children.iter().sorted_by_key(|x| x.1.inclusive) {
            self.stack.push_back(s.1);
        }
        return Some((state.data, state.exclusive, state.depth));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_order_matters() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        // assert_eq!(map.insert(Stack { frames: vec![1] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![2] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![3] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![1] }), false);
        // assert_eq!(map.insert(Stack { frames: vec![1, 2, 3] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![3, 2, 1] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![1, 1, 1] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![1, 3, 2] }), true);
        // assert_eq!(map.insert(Stack { frames: vec![1, 2, 3] }), false);

        // Exclusive weights
        map.insert(vec![1, 2, 3], 2);
        map.insert(vec![1, 2, 4], 12);
        map.insert(vec![1, 3, 1], 5);
        map.insert(vec![1, 3, 2], 1);
        map.insert(vec![2, 1, 2], 6);
        map.insert(vec![2, 1, 3], 34);
        map.insert(vec![2, 1], 13);

        // let mut vec: Vec<_> = map.into_iter().collect();
        // vec.sort_by(|a, b|
        // {
        //     a.0.cmp(&b.0)
        // });

        // for i in vec {
        //     println!(">>> {:?} - {}", i.0, i.1);
        // }

        let new_from_flat = TreeNode::new_from_flat(map);

        println!(">>> {:?}", new_from_flat);
        
        // for x in new_from_flat.iter() {
        //     println!("{:?}", x.1.inclusive);
        // }

        for x in Iter::new(&new_from_flat) {
            println!("{:?}", x);
        }

        assert_eq!(vec![1, 3] > vec![1, 2, 4], true);
        assert_eq!(vec![1, 2, 3] < vec![1, 2, 4], true);
        assert_eq!(vec![1, 2, 3, 2] < vec![1, 2, 4], true);
        assert_eq!(vec![1, 2, 3] < vec![1, 2, 3, 2], true);

        // [1, 2, 3]
        // [1, 2, 3, 2]
        // [1, 2, 4]
        // [1, 3, 4]

        // All stacks = sum of counts of all entries
        // Sorting by count : 
        /*
        
        [1, 2] 2
        [1, 2, 3] 12
        [1, 2, 4] 3
        [1, 2, 5, 6] 8
        [1, 2, 5, 7] 1000

        [1, 2] 2 --- ?
        [1, 2, 5, 7] 1000
        [1, 2, 5, 6] 8
        [1, 2, 3] 12
        [1, 2, 4] 3

        build tree from this flat stuff ?
        
         */
    }
}