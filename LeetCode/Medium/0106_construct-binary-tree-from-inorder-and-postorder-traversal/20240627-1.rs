use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_index_map = create_index_map(&inorder);
        build(
            0,
            inorder.len() as i32 - 1,
            &mut postorder,
            &inorder_index_map,
        )
    }
}

fn build(
    inorder_l: i32,
    inorder_r: i32,
    postorder: &mut Vec<i32>,
    inorder_index_map: &HashMap<i32, i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder_l > inorder_r {
        return None;
    }

    let mut root = TreeNode::new(postorder.pop().unwrap());
    let index = *inorder_index_map.get(&root.val).unwrap();

    root.right = build(index + 1, inorder_r, postorder, inorder_index_map);
    root.left = build(inorder_l, index - 1, postorder, inorder_index_map);

    Some(Rc::new(RefCell::new(root)))
}

fn create_index_map(arr: &[i32]) -> HashMap<i32, i32> {
    let mut result = HashMap::new();

    for i in 0..arr.len() {
        result.insert(arr[i], i as i32);
    }

    result
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
