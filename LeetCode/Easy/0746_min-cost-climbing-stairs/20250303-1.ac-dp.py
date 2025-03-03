class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        dp = [c for c in cost]
        dp.append(0)

        for i in range(2, len(dp)):
            dp[i] = min(dp[i - 1], dp[i - 2]) + dp[i]

        return dp[-1]