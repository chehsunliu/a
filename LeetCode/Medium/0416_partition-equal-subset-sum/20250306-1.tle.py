class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        dp = [0 for _ in nums]

        for i in reversed(range(len(nums))):
            for j in range(i + 1, len(nums)):
                if nums[i] < nums[j]:
                    dp[i] = max(dp[i], dp[j])
            dp[i] += 1

        return max(dp)