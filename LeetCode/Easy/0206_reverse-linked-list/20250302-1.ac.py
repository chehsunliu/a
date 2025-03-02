# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None

        stack = []
        ptr = head
        while ptr is not None:
            stack.append(ptr)
            ptr = ptr.next

        ptr = stack.pop()
        dummy = ptr
        while len(stack) > 0:
            ptr.next = stack.pop()
            ptr = ptr.next
            ptr.next = None

        return dummy
