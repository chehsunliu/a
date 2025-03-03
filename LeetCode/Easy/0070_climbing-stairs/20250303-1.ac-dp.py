class Solution:
    def climbStairs(self, n: int) -> int:
        dp = {}
        return self.f(n, dp)

    def f(self, n: int, dp: dict[int, int]) -> int:
        if n == 0:
            return 1
        if n < 0:
            return 0

        if n in dp:
            return dp[n]

        v = self.f(n - 1, dp) + self.f(n - 2, dp)
        dp[n] = v
        return v
