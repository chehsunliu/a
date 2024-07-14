use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        let mut ptr = Rc::clone(root.as_mut().unwrap());

        loop {
            if val > ptr.borrow().val {
                if ptr.borrow().right.is_some() {
                    let ptr_next = Rc::clone(ptr.borrow_mut().right.as_mut().unwrap());
                    ptr = ptr_next;
                } else {
                    ptr.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    break;
                }
            } else {
                if ptr.borrow().left.is_some() {
                    let ptr_next = Rc::clone(ptr.borrow_mut().left.as_mut().unwrap());
                    ptr = ptr_next;
                } else {
                    ptr.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    break;
                }
            }
        }

        root
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
