impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes = vec![];
        let mut ptr = head;

        while let Some(mut v) = ptr {
            ptr = v.next.take();
            nodes.push(Some(v));
        }

        if nodes.is_empty() {
            return ptr;
        }

        for i in 0..(nodes.len() - 1) {
            let v = nodes[i].take();
            nodes[i + 1].as_mut().map(|x| x.next = v);
        }

        nodes.last_mut().unwrap().take()
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
