impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp_node = Box::new(ListNode { val: 0, next: head });
        let mut ptr = &mut temp_node;

        while ptr.next.is_some() && ptr.next.as_ref().unwrap().next.is_some() {
            let mut ptr2 = ptr.next.as_mut().unwrap().next.take().unwrap();
            let mut ptr1 = ptr.next.take().unwrap();

            ptr1.next = ptr2.next.take();
            ptr2.next = Some(ptr1);
            ptr.next = Some(ptr2);

            ptr = ptr.next.as_mut().unwrap().next.as_mut().unwrap();
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
