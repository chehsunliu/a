impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp_node = Box::new(ListNode { val: 0, next: head });
        let mut ptr = &mut temp_node;

        while let Some(mut ptr1) = ptr.next.take() {
            if let Some(mut ptr2) = ptr1.next.take() {
                ptr1.next = ptr2.next.take();
                ptr2.next = Some(ptr1);
                ptr.next = Some(ptr2);

                ptr = ptr.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                ptr.next = Some(ptr1);
                break;
            }
        }

        temp_node.next
    }
}

struct Solution;

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
