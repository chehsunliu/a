class Solution:
    def findTargetSumWays(self, nums: list[int], target: int) -> int:
        self.nums = nums
        self.target = target
        return self.f(0, 0)
    
    def f(self, index: int, result: int) -> int:
        if index == len(self.nums):
            return 1 if result == self.target else 0
        
        return self.f(index + 1, result + self.nums[index]) + self.f(index + 1, result - self.nums[index])
        