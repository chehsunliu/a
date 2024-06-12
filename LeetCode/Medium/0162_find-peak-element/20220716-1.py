class Solution:
    def findPeakElement(self, nums: list[int]) -> int:
        l, r = 0, len(nums) - 1
        
        while l <= r:
            m = (l + r) // 2
            
            if self.get_nums(nums, m) > self.get_nums(nums, m - 1) and self.get_nums(nums, m) > self.get_nums(nums, m + 1):
                return m
            elif self.get_nums(nums, m) > self.get_nums(nums, m + 1):
                r = m - 1
            else:
                l = m + 1
            
    def get_nums(self, nums: list[int], index: int) -> int:
        if 0 <= index < len(nums):
            return nums[index]
        
        return -sys.maxsize
