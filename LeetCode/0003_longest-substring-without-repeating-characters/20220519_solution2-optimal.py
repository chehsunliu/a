class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        max_length = 0
        i = 0
        records = {}
        
        for j, c in enumerate(s):
            if c in records:
                i = max(i, records[c] + 1)
            
            records[c] = j
            max_length = max(max_length, j - i + 1)
            
        return max_length