use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut parents: HashMap<usize, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut leafs: HashMap<usize, Rc<RefCell<TreeNode>>> = HashMap::new();

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.unwrap());

        while let Some(node) = queue.pop_front() {
            if let Some(node_left) = &node.borrow().left {
                parents.insert(node_left.as_ptr() as usize, Rc::clone(&node));
                queue.push_back(Rc::clone(node_left));
            }

            if let Some(node_right) = &node.borrow().right {
                parents.insert(node_right.as_ptr() as usize, Rc::clone(&node));
                queue.push_back(Rc::clone(node_right));
            }

            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                leafs.insert(node.as_ptr() as usize, Rc::clone(&node));
            }
        }

        let mut count = 0;

        for (&_, leaf) in &leafs {
            let mut visisted: HashSet<usize> = HashSet::new();
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
            queue.push_back((Rc::clone(leaf), 0));

            while let Some((node, d)) = queue.pop_front() {
                let id = node.as_ptr() as usize;
                if visisted.contains(&id) || d > distance {
                    continue;
                }

                visisted.insert(id);

                if d != 0 && leafs.contains_key(&id) {
                    count += 1;
                }

                parents
                    .get(&id)
                    .map(|t| queue.push_back((Rc::clone(t), d + 1)));
                node.borrow()
                    .left
                    .as_ref()
                    .map(|t| queue.push_back((Rc::clone(t), d + 1)));
                node.borrow()
                    .right
                    .as_ref()
                    .map(|t| queue.push_back((Rc::clone(t), d + 1)));
            }
        }

        count / 2
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
