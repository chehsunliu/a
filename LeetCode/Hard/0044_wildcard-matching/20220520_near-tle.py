class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        self.s = s
        self.pattern = p
        self.dp = [[None for j in range(len(self.pattern) + 1)] for i in range(len(self.s) + 1)]
        
        return self.f(0, 0)
    
    def f(self, i: int, j: int):
        if self.dp[i][j] is not None:
            return self.dp[i][j]
        
        if i == len(self.s) and j == len(self.pattern):
            self.dp[i][j] = True
            return True
        elif i == len(self.s):
            answer = True
            for j_tmp in range(j, len(self.pattern)):
                if self.dp[i][j_tmp] is not None:
                    answer = self.dp[i][j_tmp]
                    break

                if self.pattern[j_tmp] != "*":
                    answer = False
                    break

            self.dp[i][j] = answer
            return answer
        elif j == len(self.pattern):
            self.dp[i][j] = False
            return False
        
        c = self.s[i]
        p = self.pattern[j]
        
        if p == "*":
            answer = self.f(i, j + 1) or self.f(i + 1, j)
        
        elif p == "?":
            answer = self.f(i + 1, j + 1)
        
        else:
            if p != c:
                answer = False
            else:
                answer = self.f(i + 1, j + 1)
                
        self.dp[i][j] = answer
        return answer