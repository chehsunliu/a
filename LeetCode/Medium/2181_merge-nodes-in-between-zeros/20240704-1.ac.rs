impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut ptr = &mut dummy;

        let mut value = 0;
        while let Some(node) = head {
            if node.val == 0 {
                if value != 0 {
                    ptr.next = Some(Box::new(ListNode::new(value)));
                    ptr = ptr.next.as_mut().unwrap();
                    value = 0;
                }
            } else {
                value += node.val;
            }

            head = node.next;
        }

        dummy.next
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
