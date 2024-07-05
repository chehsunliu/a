use std::cmp::min;
use std::collections::VecDeque;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut critical_points: Vec<usize> = vec![];
        let mut previous_nodes: VecDeque<&Box<ListNode>> = VecDeque::new();
        let mut index: usize = 0;

        let mut current_head = head.as_ref();
        while let Some(current_node) = current_head {
            if previous_nodes.len() == 3 {
                previous_nodes.pop_front();
            }

            previous_nodes.push_back(current_node);
            if previous_nodes.len() == 3 {
                let node_left = *previous_nodes.front().unwrap();
                let node_middle = node_left.next.as_ref().unwrap();
                let node_right = current_node;

                if (node_left.val > node_middle.val && node_right.val > node_middle.val)
                    || (node_left.val < node_middle.val && node_right.val < node_middle.val)
                {
                    critical_points.push(index - 1);
                }
            }

            current_head = current_node.next.as_ref();
            index += 1;
        }

        if critical_points.len() < 2 {
            return vec![-1, -1];
        }

        let max_d = critical_points[critical_points.len() - 1] - critical_points[0];
        let mut min_d = critical_points[critical_points.len() - 1] - critical_points[0];

        for i in 1..critical_points.len() {
            min_d = min(min_d, critical_points[i] - critical_points[i - 1]);
        }

        vec![min_d as i32, max_d as i32]
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
