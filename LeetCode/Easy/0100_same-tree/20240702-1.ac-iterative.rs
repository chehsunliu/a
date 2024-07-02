use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut queue: VecDeque<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> =
            VecDeque::new();
        queue.push_back((p, q));

        while let Some((n1, n2)) = queue.pop_front() {
            if let (Some(n1), Some(n2)) = (&n1, &n2) {
                let (n1, n2) = (n1.borrow(), n2.borrow());

                if n1.val != n2.val {
                    return false;
                }

                queue.push_back((
                    n1.left.as_ref().map(|n| Rc::clone(n)),
                    n2.left.as_ref().map(|n| Rc::clone(n)),
                ));
                queue.push_back((
                    n1.right.as_ref().map(|n| Rc::clone(n)),
                    n2.right.as_ref().map(|n| Rc::clone(n)),
                ));
            } else {
                if (n1.is_none() && !n2.is_none()) || (!n1.is_none() && n2.is_none()) {
                    return false;
                }
            }
        }

        true
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
