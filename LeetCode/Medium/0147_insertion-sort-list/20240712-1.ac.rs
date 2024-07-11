impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr: Vec<i32> = vec![];

        while let Some(unboxed) = head {
            arr.push(unboxed.val);
            head = unboxed.next;
        }

        arr.sort();

        let mut head = Box::new(ListNode::new(arr[0]));
        let mut ptr = &mut head;

        for v in &arr[1..] {
            ptr.next = Some(Box::new(ListNode::new(*v)));
            ptr = ptr.next.as_mut().unwrap();
        }

        Some(head)
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
    #[test]
    fn basic() {}
}
