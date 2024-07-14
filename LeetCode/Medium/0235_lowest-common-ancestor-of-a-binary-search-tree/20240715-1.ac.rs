use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr = root.as_ref().map(|t| Rc::clone(t)).unwrap();
        let (p, q) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
        // Make sure p < q.
        let (p, q) = if p < q { (p, q) } else { (q, p) };

        loop {
            let value = ptr.borrow().val;

            if value == p || value == q {
                return Some(ptr);
            }

            if p < value && value < q {
                return Some(ptr);
            }

            if q < value {
                let ptr_tmp = ptr.borrow().left.as_ref().map(|t| Rc::clone(t)).unwrap();
                ptr = ptr_tmp;
            } else {
                let ptr_tmp = ptr.borrow().right.as_ref().map(|t| Rc::clone(t)).unwrap();
                ptr = ptr_tmp;
            }
        }
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

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
