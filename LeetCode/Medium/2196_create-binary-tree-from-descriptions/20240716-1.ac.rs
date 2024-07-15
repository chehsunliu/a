use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let descriptions = descriptions
            .into_iter()
            .map(|d| Description {
                parent: d[0],
                child: d[1],
                is_left: d[2] != 0,
            })
            .collect::<Vec<Description>>();

        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut has_been_child: HashSet<i32> = HashSet::new();

        for d in &descriptions {
            let parent = if let Some(p) = nodes.get(&d.parent) {
                Rc::clone(p)
            } else {
                let p = Rc::new(RefCell::new(TreeNode::new(d.parent)));
                nodes.insert(d.parent, Rc::clone(&p));
                p
            };

            let child = if let Some(c) = nodes.get(&d.child) {
                Rc::clone(c)
            } else {
                let c = Rc::new(RefCell::new(TreeNode::new(d.child)));
                nodes.insert(d.child, Rc::clone(&c));
                c
            };

            if d.is_left {
                parent.borrow_mut().left = Some(child);
            } else {
                parent.borrow_mut().right = Some(child);
            }

            has_been_child.insert(d.child);
        }

        for (k, v) in nodes {
            if !has_been_child.contains(&k) {
                return Some(v);
            }
        }

        unreachable!();
    }
}

#[derive(Debug)]
pub struct Description {
    parent: i32,
    child: i32,
    is_left: bool,
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
