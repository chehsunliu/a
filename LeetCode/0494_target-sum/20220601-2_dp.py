class Solution:
    def findTargetSumWays(self, nums: list[int], target: int) -> int:
        self.nums = nums
        self.dp = [None for _ in nums]
        self.f(0)
        return self.dp[0][target] if target in self.dp[0] else 0
    
    def f(self, index: int):
        if index == len(self.nums):
            return {0: 1}
        
        if self.dp[index] is not None:
            return self.dp[index]
        
        dp = {}

        for value, count in self.f(index + 1).items():
            if value + self.nums[index] not in dp:
                dp[value + self.nums[index]] = 0
            if value - self.nums[index] not in dp:
                dp[value - self.nums[index]] = 0
                
            dp[value + self.nums[index]] += count
            dp[value - self.nums[index]] += count
            
        self.dp[index] = dp
        return dp
