class Solution:
    def maxSubArray(self, nums: list[int]) -> int:
        dp = nums[:]
        
        for i in range(len(dp) - 2, -1, -1):
            dp[i] = max(dp[i], dp[i] + dp[i + 1])
            
        return max(dp)
