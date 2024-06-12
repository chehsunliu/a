class Solution:
    def findDuplicate(self, nums: list[int]) -> int:
        nums = list(sorted(nums))
        for i in range(len(nums) - 1):
            if nums[i] == nums[i + 1]:
                return nums[i]
