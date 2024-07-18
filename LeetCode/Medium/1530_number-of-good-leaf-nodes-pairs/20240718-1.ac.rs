use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut leafs = HashMap::new();
        create_leafs(None, root.unwrap(), &mut 0, &mut leafs);

        let mut pairs: HashSet<(i32, i32)> = HashSet::new();

        for leaf in leafs.values() {
            let mut visited: HashSet<i32> = HashSet::new();
            let mut queue: VecDeque<(Rc<RefCell<Node>>, i32)> = VecDeque::new();

            queue.push_back((Rc::clone(leaf), 0));

            while let Some((node, d)) = queue.pop_front() {
                if visited.contains(&node.borrow().id) || d > distance {
                    continue;
                }

                visited.insert(node.borrow().id);

                if leaf.borrow().id != node.borrow().id && leafs.contains_key(&node.borrow().id) {
                    if leaf.borrow().id > node.borrow().id {
                        pairs.insert((node.borrow().id, leaf.borrow().id));
                    } else {
                        pairs.insert((leaf.borrow().id, node.borrow().id));
                    }
                }

                for neighbor in &node.borrow().neighbors {
                    queue.push_back((Rc::clone(neighbor), d + 1));
                }
            }
        }

        pairs.len() as i32
    }
}

#[derive(Debug)]
struct Node {
    id: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: i32) -> Self {
        Self {
            id,
            neighbors: vec![],
        }
    }
}

fn create_leafs(
    node_p: Option<Rc<RefCell<Node>>>,
    t_node: Rc<RefCell<TreeNode>>,
    id: &mut i32,
    leafs: &mut HashMap<i32, Rc<RefCell<Node>>>,
) {
    let node = Rc::new(RefCell::new(Node::new(*id)));
    *id += 1;

    if let Some(n_p) = node_p {
        n_p.borrow_mut().neighbors.push(Rc::clone(&node));
        node.borrow_mut().neighbors.push(n_p);
    }

    let t_node = t_node.borrow();
    if let (Some(t_n_l), Some(t_n_r)) = (t_node.left.as_ref(), t_node.right.as_ref()) {
        create_leafs(Some(Rc::clone(&node)), Rc::clone(t_n_l), id, leafs);
        create_leafs(Some(Rc::clone(&node)), Rc::clone(t_n_r), id, leafs);
    } else if let Some(t_n_l) = t_node.left.as_ref() {
        create_leafs(Some(Rc::clone(&node)), Rc::clone(t_n_l), id, leafs);
    } else if let Some(t_n_r) = t_node.right.as_ref() {
        create_leafs(Some(Rc::clone(&node)), Rc::clone(t_n_r), id, leafs);
    } else {
        let id = node.borrow().id;
        leafs.insert(id, node);
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
