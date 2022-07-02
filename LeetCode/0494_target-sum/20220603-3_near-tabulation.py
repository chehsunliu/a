class Solution:
    def findTargetSumWays(self, nums: list[int], target: int) -> int:
        dp = [{} for _ in range(len(nums) + 1)]
        dp[len(nums)] = {0: 1}
        
        for i in range(len(nums) - 1, -1, -1):
            for value, count in dp[i + 1].items():
                vs = [value + nums[i], value - nums[i]]
                for v in vs:
                    if v not in dp[i]:
                        dp[i][v] = 0
                    dp[i][v] += count
        
        return dp[0][target] if target in dp[0] else 0