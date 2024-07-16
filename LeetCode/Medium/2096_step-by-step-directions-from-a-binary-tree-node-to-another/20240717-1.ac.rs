use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut vs = search(start_value, root.as_ref().map(|t| Rc::clone(t))).unwrap();
        let mut vd = search(dest_value, root.as_ref().map(|t| Rc::clone(t))).unwrap();

        let mut last_vs1 = None;
        let mut last_vd1 = None;
        while let (Some(vs1), Some(vd1)) = (vs.last(), vd.last()) {
            if vs1.0 != vd1.0 {
                break;
            }

            last_vs1 = vs.pop();
            last_vd1 = vd.pop();
        }

        let last_vs1 = last_vs1.unwrap();
        let last_vd1 = last_vd1.unwrap();

        vs.push(last_vs1);
        vd.push(last_vd1);

        let mut ans = "".to_string();

        for i in (0..vs.len()).rev() {
            if vs[i].1 == 'X' {
                break;
            }

            ans.push('U');
        }

        for i in (0..vd.len()).rev() {
            if vd[i].1 == 'X' {
                break;
            }

            ans.push(vd[i].1);
        }

        ans
    }
}

fn search(value: i32, node: Option<Rc<RefCell<TreeNode>>>) -> Option<Vec<(i32, char)>> {
    if let Some(n) = node {
        if n.borrow().val == value {
            Some(vec![(value, 'X')])
        } else {
            search(value, n.borrow().left.as_ref().map(|t| Rc::clone(t)))
                .map(|mut vs| {
                    vs.push((n.borrow().val, 'L'));
                    vs
                })
                .or_else(|| {
                    search(value, n.borrow().right.as_ref().map(|t| Rc::clone(t))).map(|mut vs| {
                        vs.push((n.borrow().val, 'R'));
                        vs
                    })
                })
        }
    } else {
        None
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
