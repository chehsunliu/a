class Solution:
    def countSubstrings(self, s: str) -> int:
        ans = 0
        for i in range(len(s)):
            ans += self.f(s, i, i)
            if i + 1 < len(s) and s[i] == s[i + 1]:
                ans += self.f(s, i, i + 1)
        return ans

    def f(self, s: str, i: int, j: int) -> int:
        count = 0
        while i >= 0 and j < len(s) and s[i] == s[j]:
            i -= 1
            j += 1
            count += 1
        return count