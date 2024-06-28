use std::cell::RefCell;
use std::collections::HashMap;
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
        let mut s1 = String::new();
        let mut s2 = String::new();

        inorder(root.as_ref(), &mut 0, &mut s1);
        postorder(root.as_ref(), &mut 0, &mut s2);

        let mut ans = String::new();
        ans.push_str(&s1);
        ans.push_str("| ");
        ans.push_str(&s2);

        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (s1, s2) = data.trim().split_once('|').unwrap();
        let (inorder, mut postorder) = (parse(s1), parse(s2));

        let inorder_index_map = create_index_map(&inorder);
        build(
            0,
            inorder.len() as i32 - 1,
            &mut postorder,
            &inorder_index_map,
        )
    }
}

fn inorder(node: Option<&Rc<RefCell<TreeNode>>>, index: &mut i32, output: &mut String) {
    if node.is_none() {
        return;
    }
    *index += 1;
    let current_index = *index;

    inorder(node.unwrap().borrow().left.as_ref(), index, output);

    output.push_str(current_index.to_string().as_str());
    output.push(':');
    output.push_str(node.unwrap().borrow().val.to_string().as_str());
    output.push(' ');

    inorder(node.unwrap().borrow().right.as_ref(), index, output);
}

fn postorder(node: Option<&Rc<RefCell<TreeNode>>>, index: &mut i32, output: &mut String) {
    if node.is_none() {
        return;
    }
    *index += 1;
    let current_index = *index;

    postorder(node.unwrap().borrow().left.as_ref(), index, output);
    postorder(node.unwrap().borrow().right.as_ref(), index, output);

    output.push_str(current_index.to_string().as_str());
    output.push(':');
    output.push_str(node.unwrap().borrow().val.to_string().as_str());
    output.push(' ');
}

// "2:9 1:3 4:15 3:20 5:7 "
fn parse(s: &str) -> Vec<(i32, i32)> {
    s.trim()
        .split_whitespace()
        .map(|token| {
            let (index, value) = token.split_once(':').unwrap();
            (index.parse::<i32>().unwrap(), value.parse::<i32>().unwrap())
        })
        .collect::<Vec<(i32, i32)>>()
}

fn build(
    inorder_l: i32,
    inorder_r: i32,
    postorder: &mut Vec<(i32, i32)>,
    inorder_index_map: &HashMap<(i32, i32), i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder_l > inorder_r {
        return None;
    }

    let value = postorder.pop().unwrap();
    let mut root = TreeNode::new(value.1);
    let index = *inorder_index_map.get(&value).unwrap();

    root.right = build(index + 1, inorder_r, postorder, inorder_index_map);
    root.left = build(inorder_l, index - 1, postorder, inorder_index_map);

    Some(Rc::new(RefCell::new(root)))
}

fn create_index_map(arr: &[(i32, i32)]) -> HashMap<(i32, i32), i32> {
    let mut result = HashMap::new();

    for i in 0..arr.len() {
        result.insert(arr[i], i as i32);
    }

    result
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
