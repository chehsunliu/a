# 10 01 10 10 10
#  l     m     r
#  l  m  r

class Solution:
    def findMin(self, nums: list[int]) -> int:
        for i in range(len(nums) - 1):
            if nums[i] > nums[i + 1]:
                return nums[i + 1]
            
        return nums[0]