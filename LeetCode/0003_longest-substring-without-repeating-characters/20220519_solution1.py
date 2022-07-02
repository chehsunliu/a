class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        max_length = 0
        base = 0
        records = {}
        
        for i, c in enumerate(s):
            if c in records:
                max_length = max(max_length, i - base)
                
                for j in range(base, records[c]):
                    records.pop(s[j])
                
                base = records[c] + 1
                records[c] = i
            
            else:
                records[c] = i
                
        return max(max_length, len(s) - base)