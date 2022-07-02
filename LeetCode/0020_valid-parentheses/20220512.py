class Solution:
    def isValid(self, s: str) -> bool:
        mapping = {
            "(": ")",
            "[": "]",
            "{": "}",
        }
        stack = []
        
        for c in s:
            if c in mapping:
                stack.append(mapping[c])
                
            else:
                if not stack or stack.pop() != c:
                    return False
                
        return len(stack) == 0