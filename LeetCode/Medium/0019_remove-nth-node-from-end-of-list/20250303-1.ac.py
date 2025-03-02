# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        _, head = self.f(head, n)
        return head

    def f(self, head: ListNode | None, n: int):
        if head is None:
            return 1, None

        i, node = self.f(head.next, n)

        if i == n:
            return i + 1, node
        else:
            head.next = node
            return i + 1, head