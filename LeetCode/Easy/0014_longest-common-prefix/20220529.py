class Solution:
    def longestCommonPrefix(self, strs: list[str]) -> str:
        i = 0
        ans = ""
        
        while True:
            s0 = strs[0]
            
            for s in strs:
                if i >= len(s) or s0[i] != s[i]:
                    return ans
                
            i += 1
            ans = s0[:i]
