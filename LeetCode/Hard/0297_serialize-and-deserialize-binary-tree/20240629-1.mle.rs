use std::cell::RefCell;
use std::collections::VecDeque;
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
        let count = count_nodes(root.as_ref());
        let mut current_count = 0;

        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root.as_ref().map(|node| Rc::clone(node)));

        let mut values: Vec<Option<i32>> = vec![];

        while current_count < count {
            let node = queue.pop_front().unwrap();
            if let Some(node) = node {
                values.push(Some(node.borrow().val));

                current_count += 1;
                queue.push_back(node.borrow().left.as_ref().map(|node| Rc::clone(node)));
                queue.push_back(node.borrow().right.as_ref().map(|node| Rc::clone(node)));
            } else {
                values.push(None);

                queue.push_back(None);
                queue.push_back(None);
            }
        }

        let s = values
            .iter()
            .map(|v| v.map_or("null".to_string(), |vv| vv.to_string()))
            .collect::<Vec<String>>()
            .join(",");

        let mut ans = "".to_string();
        ans.push('[');
        ans.push_str(&s);
        ans.push(']');
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let data = &data[1..data.len() - 1];
        if data.is_empty() {
            return None;
        }

        let mut nodes = data
            .split(',')
            .map(|s| match s {
                "null" => None,
                _ => Some(Rc::new(RefCell::new(TreeNode::new(
                    s.parse::<i32>().unwrap(),
                )))),
            })
            .collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>();

        for i in 0..nodes.len() {
            if let Some(node) = nodes[i].as_ref() {
                if i * 2 + 1 < nodes.len() {
                    let l = nodes[i * 2 + 1].as_ref();
                    node.borrow_mut().left = l.map(|c| Rc::clone(c));
                }

                if i * 2 + 2 < nodes.len() {
                    let r = nodes[i * 2 + 2].as_ref();
                    node.borrow_mut().right = r.map(|c| Rc::clone(c));
                }
            }
        }

        nodes[0].as_ref().map(|node| Rc::clone(node))
    }
}

fn count_nodes(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    if node.is_none() {
        return 0;
    }

    let node = node.unwrap().borrow();
    return count_nodes(node.left.as_ref()) + count_nodes(node.right.as_ref()) + 1;
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
