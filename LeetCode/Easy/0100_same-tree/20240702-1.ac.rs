use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (Some(p), Some(q)) = (&p, &q) {
            if p.borrow().val != q.borrow().val {
                false
            } else {
                Self::is_same_tree(
                    p.borrow().left.as_ref().map(|n| Rc::clone(n)),
                    q.borrow().left.as_ref().map(|n| Rc::clone(n)),
                ) && Self::is_same_tree(
                    p.borrow().right.as_ref().map(|n| Rc::clone(n)),
                    q.borrow().right.as_ref().map(|n| Rc::clone(n)),
                )
            }
        } else {
            if p.is_none() && q.is_none() {
                true
            } else {
                false
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
