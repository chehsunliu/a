# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        ptr1 = head
        ptr2 = head

        while ptr1 and ptr2 and ptr1.next and ptr2.next and ptr2.next.next:
            ptr1 = ptr1.next
            ptr2 = ptr2.next.next
            if id(ptr1) == id(ptr2):
                return True

        return False