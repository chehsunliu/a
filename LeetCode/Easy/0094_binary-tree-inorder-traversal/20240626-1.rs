use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        f(root.as_ref(), &mut ans);
        ans
    }
}

fn f(node: Option<&Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }

    let node = node.unwrap().borrow();
    f(node.left.as_ref(), ans);
    ans.push(node.val);
    f(node.right.as_ref(), ans);
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
