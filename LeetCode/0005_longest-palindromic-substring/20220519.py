class Solution:
    def longestPalindrome(self, s: str) -> str:
        max_length = 1
        ans = s[0]

        for i in range(len(s)):
            length0, s0 = self.f(s, l=i, r=i)
            if length0 > max_length:
                max_length = length0
                ans = s0
                
            length1, s1 = self.f(s, l=i, r=i+1)
            if length1 > max_length:
                max_length = length1
                ans = s1
                
        return ans
    
    def f(self, s: str, l: int, r: int) -> tuple[int, str]:
        max_length = 0
        ans = ""
        
        while l >= 0 and r < len(s) and s[l] == s[r]:
            max_length = r - l + 1
            ans = s[l:r+1]
            
            l -= 1
            r += 1
        
        return max_length, ans
        