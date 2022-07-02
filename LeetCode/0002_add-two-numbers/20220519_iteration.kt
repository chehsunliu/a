/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun addTwoNumbers(l1: ListNode?, l2: ListNode?): ListNode? {
        val l = ListNode(0)
        var lp: ListNode? = l
        
        var lp1 = l1
        var lp2 = l2
        
        while (lp != null) {
            val v1 = lp1?.`val` ?: 0
            val v2 = lp2?.`val` ?: 0
            
            lp1 = lp1?.next
            lp2 = lp2?.next
            
            val sum = lp.`val` + v1 + v2
            
            lp.`val` = sum % 10
            val carry = sum / 10
            
            if (lp1 != null || lp2 != null || carry > 0) {
                lp.next = ListNode(carry)
            }
            
            lp = lp.next
        }
        
        return l
    }
}