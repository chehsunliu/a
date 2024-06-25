use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        f(&root.as_ref().unwrap().borrow(), target_sum)
    }
}

fn f(node: &TreeNode, target: i32) -> bool {
    if node.left.is_none() && node.right.is_none() {
        return target == node.val;
    }

    let mut ans = false;
    if let Some(v) = node.left.as_ref() {
        ans |= f(&v.borrow(), target - node.val);
    }
    if let Some(v) = node.right.as_ref() {
        ans |= f(&v.borrow(), target - node.val);
    }

    ans
}

struct Solution;

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
