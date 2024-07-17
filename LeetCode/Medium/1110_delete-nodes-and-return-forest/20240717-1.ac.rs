use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(PartialEq)]
enum ChildDirection {
    Left,
    Right,
}

impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if root.is_none() {
            return vec![];
        }

        let to_delete: HashSet<i32> = HashSet::from_iter(to_delete);

        let mut ans: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut queue: VecDeque<(
            Option<(Rc<RefCell<TreeNode>>, ChildDirection)>,
            Rc<RefCell<TreeNode>>,
        )> = VecDeque::new();

        let root = root.unwrap();
        ans.insert(root.borrow().val, Rc::clone(&root));
        queue.push_back((None, root));

        while let Some((node_parent, n)) = queue.pop_front() {
            if to_delete.contains(&n.borrow().val) {
                if let Some((n_parent, direction)) = node_parent {
                    if direction == ChildDirection::Left {
                        n_parent.borrow_mut().left.take();
                    } else {
                        n_parent.borrow_mut().right.take();
                    }
                }

                ans.remove(&n.borrow().val);
            }

            if let Some(n_left) = n.borrow().left.as_ref() {
                queue.push_back((
                    Some((Rc::clone(&n), ChildDirection::Left)),
                    Rc::clone(n_left),
                ));

                if to_delete.contains(&n.borrow().val) {
                    ans.insert(n_left.borrow().val, Rc::clone(n_left));
                }
            }
            if let Some(n_right) = n.borrow().right.as_ref() {
                queue.push_back((
                    Some((Rc::clone(&n), ChildDirection::Right)),
                    Rc::clone(n_right),
                ));

                if to_delete.contains(&n.borrow().val) {
                    ans.insert(n_right.borrow().val, Rc::clone(n_right));
                }
            }
        }

        ans.into_values().map(|t| Some(t)).collect()
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
