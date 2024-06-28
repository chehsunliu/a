use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v_p = search(root.as_ref(), p.as_ref().unwrap().borrow().val).unwrap();
        let mut v_q = search(root.as_ref(), q.as_ref().unwrap().borrow().val).unwrap();

        v_p.reverse();
        v_q.reverse();

        let mut i = 0;
        let mut ans = None;
        while i < v_p.len() && i < v_q.len() && v_p[i].borrow().val == v_q[i].borrow().val {
            ans = Some(Rc::clone(&v_p[i]));
            i += 1;
        }

        ans
    }
}

fn search(node: Option<&Rc<RefCell<TreeNode>>>, value: i32) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
    if node.is_none() {
        return None;
    }

    let node = node.unwrap();
    if node.borrow().val == value {
        return Some(vec![Rc::clone(node)]);
    }

    if let Some(mut path) = search(node.borrow().left.as_ref(), value) {
        path.push(Rc::clone(node));
        return Some(path);
    }

    if let Some(mut path) = search(node.borrow().right.as_ref(), value) {
        path.push(Rc::clone(node));
        return Some(path);
    }

    None
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
