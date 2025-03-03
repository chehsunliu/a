class Solution:
    def rob(self, nums: List[int]) -> int:
        dp = [v for v in nums] + [0]

        for i in reversed(range(len(nums) - 1)):
            dp[i] = max(dp[i] + dp[i + 2], dp[i + 1])

        return dp[0]

# 0 1 2 3 4 5
# 2 1 3 1 1 9
# ^   ^     ^

# f(5) = 9
# f(4) = max(1            , f(5) = 9) = 9
# f(3) = max(1 + f(5) = 10, f(4) = 9) = 10
# f(2) = max(3 + f(4) = 12, f(3) = 10) = 12
# f(1) = max(1 + f(3) = 11, f(2) = 12) = 12
# f(0) = max(2 + f(2) = 14, f(1) = 12) = 14