class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if len(needle) == 0:
            return 0

        for i in range(len(haystack)):
            if len(haystack) - i < len(needle):
                return -1
            
            valid = all([needle[j] == haystack[i + j] for j in range(len(needle))])
            if valid:
                return i
            
        return -1
                
            