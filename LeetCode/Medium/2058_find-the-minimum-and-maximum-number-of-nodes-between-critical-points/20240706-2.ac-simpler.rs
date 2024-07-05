use std::cmp::min;

impl Solution {
    pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nodes = vec![];
        while let Some(node) = head {
            nodes.push(node.val);
            head = node.next;
        }

        let mut critical_points = vec![];
        for i in 2..nodes.len() {
            let (vl, vm, vr) = (nodes[i - 2], nodes[i - 1], nodes[i]);
            if (vl > vm && vr > vm) || (vl < vm && vr < vm) {
                critical_points.push(i - 1);
            }
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
