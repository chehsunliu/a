use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;

        f(root.as_ref(), &mut count);

        count
    }
}

fn f(node: Option<&Rc<RefCell<TreeNode>>>, count: &mut i32) -> HashSet<i32> {
    if node.is_none() {
        return HashSet::new();
    }

    let node = node.unwrap().borrow();

    let mut s = f(node.left.as_ref(), count);
    s.extend(f(node.right.as_ref(), count));
    s.insert(node.val);

    if s.iter().count() == 1 {
        *count = *count + 1;
    }

    s
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
