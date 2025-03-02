# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        ptr = dummy

        while True:
            if list1 and list2:
                if list1.val < list2.val:
                    ptr.next = list1
                    list1 = list1.next
                    ptr = ptr.next
                    ptr.next = None
                else:
                    ptr.next = list2
                    list2 = list2.next
                    ptr = ptr.next
                    ptr.next = None
            elif list1:
                ptr.next = list1
                break

            elif list2:
                ptr.next = list2
                break

            else:
                break

        return dummy.next