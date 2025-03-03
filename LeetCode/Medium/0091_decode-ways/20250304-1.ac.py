class Solution:
    def numDecodings(self, s: str) -> int:
        dp = {}
        return self.f(s, dp)

    def f(self, s: str, dp: dict[str, int]) -> int:
        if len(s) == 0:
            return 1

        if s[0] == "0":
            return 0

        if len(s) == 1:
            return 1

        if s in dp:
            return dp[s]

        v = int(s[:2])
        v = 1 if v > 0 and v <= 26 else 0
        ans = self.f(s[:1], dp) * self.f(s[1:], dp) + v * self.f(s[2:], dp)
        dp[s] = ans
        return ans

# 11106 = 1 1106
#         11 106
# 1106 = 1 106
#        11 06
# 106 = 1 06
#       10 6