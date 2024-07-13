use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct BSTIterator {
    values: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = VecDeque::new();
        inorder_traversal(root, &mut values);
        Self { values }
    }

    fn next(&mut self) -> i32 {
        self.values.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        self.values.front().is_some()
    }
}

fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, values: &mut VecDeque<i32>) {
    match node {
        Some(node) => {
            inorder_traversal(node.borrow().left.as_ref().map(|n| Rc::clone(n)), values);
            values.push_back(node.borrow().val);
            inorder_traversal(node.borrow().right.as_ref().map(|n| Rc::clone(n)), values);
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
