use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => {
                let node_val = root.borrow().val;
                if node_val == val {
                    Some(root)
                } else if node_val > val {
                    Solution::search_bst(root.borrow_mut().left.take(), val)
                } else {
                    Solution::search_bst(root.borrow_mut().right.take(), val)
                }
            }
            None => None,
        }
    }
}

struct Solution;

// Definition for singly-linked list.
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
