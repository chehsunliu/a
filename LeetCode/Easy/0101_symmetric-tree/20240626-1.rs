use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.as_ref().unwrap().borrow();
        f(root.left.as_ref(), root.right.as_ref())
    }
}

fn f(n1: Option<&Rc<RefCell<TreeNode>>>, n2: Option<&Rc<RefCell<TreeNode>>>) -> bool {
    if (n1.is_none() && n2.is_some()) || (n1.is_some() && n2.is_none()) {
        return false;
    }

    if n1.is_none() && n2.is_none() {
        return true;
    }

    let n1 = n1.unwrap().borrow();
    let n2 = n2.unwrap().borrow();

    if n1.val != n2.val {
        return false;
    }

    f(n1.left.as_ref(), n2.right.as_ref()) && f(n1.right.as_ref(), n2.left.as_ref())
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
