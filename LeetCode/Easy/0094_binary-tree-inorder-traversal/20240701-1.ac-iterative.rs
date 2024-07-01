use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }

        let mut stack: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        let root = root.unwrap();
        stack.push_back(Rc::clone(&root));

        let mut current_node = root.borrow().left.as_ref().map(|t| Rc::clone(t));

        loop {
            match current_node {
                Some(node) => {
                    stack.push_back(Rc::clone(&node));
                    current_node = node.borrow().left.as_ref().map(|t| Rc::clone(t));
                }
                None => match stack.pop_back() {
                    Some(node) => {
                        ans.push(node.borrow().val);
                        current_node = node.borrow().right.as_ref().map(|t| Rc::clone(t));
                    }
                    None => {
                        break;
                    }
                },
            }
        }

        ans
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
