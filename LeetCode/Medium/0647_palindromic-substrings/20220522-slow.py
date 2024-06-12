class Solution:
    def countSubstrings(self, s: str) -> int:
        self.s = s
        self.answer = 0
        
        for i in range(len(self.s)):
            self.f(i, i)
            self.f(i, i + 1)
        
        return self.answer
        
    def f(self, i: int, j: int) -> None:
        if i >= len(self.s) or j >= len(self.s) or i < 0 or j < 0:
            return
        
        if self.s[i] != self.s[j]:
            return
        
        self.answer += 1
        self.f(i - 1, j + 1)
        