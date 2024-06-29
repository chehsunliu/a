use std::cell::{Ref, RefCell};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = vec![];
        serialize(root.as_ref(), &mut out);
        out.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tokens = data.split(',').collect::<VecDeque<&str>>();
        deserialize(&mut tokens)
    }
}

fn serialize(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<String>) {
    match node {
        Some(node) => {
            out.push(node.borrow().val.to_string());
            serialize(node.borrow().left.as_ref(), out);
            serialize(node.borrow().right.as_ref(), out);
        }
        None => out.push("null".to_string()),
    }
}

fn deserialize(tokens: &mut VecDeque<&str>) -> Option<Rc<RefCell<TreeNode>>> {
    let token = tokens.pop_front().unwrap();
    if token == "null" {
        return None;
    }

    let value = token.parse::<i32>().unwrap();
    let node = Rc::new(RefCell::new(TreeNode::new(value)));
    node.borrow_mut().left = deserialize(tokens);
    node.borrow_mut().right = deserialize(tokens);
    return Some(node);
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
