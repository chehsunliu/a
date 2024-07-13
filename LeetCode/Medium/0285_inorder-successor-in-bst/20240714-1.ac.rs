use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = vec![];
        inorder_traversal(root.as_ref().map(|t| Rc::clone(t)), &mut nodes);

        let mut index = 0;
        for i in 0..nodes.len() {
            if nodes[i] == p.as_ref().map(|t| Rc::clone(t)).unwrap() {
                index = i + 1;
                break;
            }
        }

        if index < nodes.len() {
            Some(Rc::clone(&nodes[index]))
        } else {
            None
        }
    }
}

fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
    match node {
        Some(node) => {
            inorder_traversal(node.borrow().left.as_ref().map(|t| Rc::clone(t)), nodes);
            nodes.push(Rc::clone(&node));
            inorder_traversal(node.borrow().right.as_ref().map(|t| Rc::clone(t)), nodes);
        }
        _ => {}
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
