class Solution:
    def decodeString(self, s: str) -> str:
        self.s = deque(s)
        return self.f()
        
    def f(self):
        if len(self.s) == 0:
            return ""
        
        if self.s[0].isalpha():
            return self.s.popleft() + self.f()
        
        if self.s[0] == "]":
            self.s.popleft()
            return ""
        
        # Must be a number
        k = 0
        while self.s[0] != "[":
            k = k * 10 + int(self.s.popleft())
            
        self.s.popleft()
            
        return k * self.f() + self.f()
