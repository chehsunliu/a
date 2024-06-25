use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        f(root.as_ref(), 1, &mut ans);
        ans
    }
}

fn f(node: Option<&Rc<RefCell<TreeNode>>>, level: usize, ans: &mut Vec<Vec<i32>>) {
    if node.is_none() {
        return;
    }

    while ans.len() < level {
        ans.push(vec![]);
    }

    let node = node.unwrap().borrow();
    ans[level - 1].push(node.val);

    f(node.left.as_ref(), level + 1, ans);
    f(node.right.as_ref(), level + 1, ans);
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
