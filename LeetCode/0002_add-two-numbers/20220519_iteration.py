# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        l = ListNode()
        p = l
        carry = 0
        
        while l1 or l2 or p:
            if l1 == None:
                v1 = 0
            else:
                v1 = l1.val
                l1 = l1.next
                
            if l2 == None:
                v2 = 0
            else:
                v2 = l2.val
                l2 = l2.next
                
            s = p.val + v1 + v2

            carry = s // 10
            s %= 10
            
            p.val = s
            if carry > 0:
                p.next = ListNode(val=carry)
            elif l1 or l2:
                p.next = ListNode()
                
            p = p.next
            
        return l