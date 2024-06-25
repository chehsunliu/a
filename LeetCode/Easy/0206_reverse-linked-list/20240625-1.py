# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

def f(node: Optional[ListNode]) -> tuple[Optional[ListNode], Optional[ListNode]]:
    if node is None:
        return None, None

    head = node
    tail = node.next
    node.next = None

    a, b = f(tail)
    if a is None:
        return head, head
    
    b.next = head
    return a, head


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        a, b = f(head)
        return a
