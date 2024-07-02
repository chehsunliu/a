use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match f(root) {
            Some((_, _, v)) => v,
            None => true,
        }
    }
}

fn f(node: Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32, bool)> {
    if node.is_none() {
        return None;
    }

    let node = node.as_ref().unwrap().borrow();
    let v_l = f(node.left.as_ref().map(|n| Rc::clone(n)));
    let v_r = f(node.right.as_ref().map(|n| Rc::clone(n)));

    if v_l.is_some() && v_r.is_some() {
        let v_l = v_l.unwrap();
        let v_r = v_r.unwrap();
        let is_valid = v_l.2 && v_r.2 && node.val > v_l.1 && node.val < v_r.0;
        Some((v_l.0, v_r.1, is_valid))
    } else if v_l.is_some() {
        let v_l = v_l.unwrap();
        let is_valid = v_l.2 && node.val > v_l.1;
        Some((v_l.0, node.val, is_valid))
    } else if v_r.is_some() {
        let v_r = v_r.unwrap();
        let is_valid = v_r.2 && node.val < v_r.0;
        Some((node.val, v_r.1, is_valid))
    } else {
        Some((node.val, node.val, true))
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

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sort_array(vec![4, 1, 2, 3]), vec![1, 2, 3, 4]);
    }
}
