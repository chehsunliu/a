impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut ptr_head = Box::new(ListNode { val: 0, next: None });
        let mut ptr_tail = ptr_head.as_mut();

        while list1.is_some() || list2.is_some() {
            let v1 = list1.as_ref().map_or(i32::MAX, |node| node.val);
            let v2 = list2.as_ref().map_or(i32::MAX, |node| node.val);

            if v1 < v2 {
                let tails = list1.as_mut().unwrap().next.take();
                ptr_tail.next = list1;
                list1 = tails;
            } else {
                let tails = list2.as_mut().unwrap().next.take();
                ptr_tail.next = list2;
                list2 = tails;
            }

            ptr_tail = ptr_tail.next.as_mut().unwrap();
        }

        ptr_head.next
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
