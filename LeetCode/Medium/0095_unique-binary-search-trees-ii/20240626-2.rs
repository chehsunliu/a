use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        f(1, n + 1)
    }
}

fn f(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if l >= r {
        return vec![None];
    }

    let mut ans = vec![];

    for i in l..r {
        let left_nodes = f(l, i);
        let right_nodes = f(i + 1, r);

        for left_node in &left_nodes {
            for right_node in &right_nodes {
                let mut node = TreeNode::new(i);
                node.left = left_node.clone();
                node.right = right_node.clone();
                ans.push(Some(Rc::new(RefCell::new(node))));
            }
        }
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
