class Solution:
    def longestPalindrome(self, s: str) -> str:
        ans = (0, "")
        for i in range(len(s)):
            ans = max(ans, self.f(s, i, i))
            if i + 1 < len(s) and s[i] == s[i + 1]:
                ans = max(ans, self.f(s, i, i + 1))
        return ans[1]

    def f(self, s: str, i: int, j: int):
        while i >= 0 and j < len(s) and s[i] == s[j]:
            i -= 1
            j += 1

        ss = s[i + 1:j]
        return (len(ss), ss)