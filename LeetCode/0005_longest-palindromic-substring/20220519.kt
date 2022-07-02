class Solution {
    fun longestPalindrome(s: String): String {
        var ans = s.substring(0, 1)
        
        (0..s.length - 1).forEach {
            val s0 = f(s, it, it)
            if (s0.length > ans.length) {
                ans = s0
            }
            
            val s1 = f(s, it, it + 1)
            if (s1.length > ans.length) {
                ans = s1
            }
        }
        
        return ans
    }
    
    fun f(s: String, l: Int, r: Int): String {
        var offset = 0
        
        while (l - offset >= 0 && r + offset < s.length && s[l - offset] == s[r + offset]) {
            offset += 1
        }
        
        offset -= 1
        
        return if (offset >= 0) s.substring(l - offset, r + offset + 1) else ""
    }
}