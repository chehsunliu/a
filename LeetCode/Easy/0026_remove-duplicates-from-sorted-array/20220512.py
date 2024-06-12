class Solution:
    def removeDuplicates(self, nums: list[int]) -> int:
        if len(nums) <= 1:
            return len(nums)

        l, r = 1, 1

        while r < len(nums):
            if nums[l - 1] < nums[r]:
                nums[l] = nums[r]
                l += 1
                r += 1
            
            else:
                r += 1
            
        return l