# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        s = l1.val + l2.val
        
        if l1.next is None and l2.next is None:
            if s // 10 > 0:
                return ListNode(val=s % 10, next=ListNode(val=s // 10))
            else:
                return ListNode(val=s % 10)
        
        if l1.next is None:
            l1.next = ListNode()
            
        if l2.next is None:
            l2.next = ListNode()
        
        l1.next.val += s // 10
        return ListNode(val=s % 10, next=self.addTwoNumbers(l1.next, l2.next))