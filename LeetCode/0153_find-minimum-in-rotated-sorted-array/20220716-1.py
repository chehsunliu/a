# 4 5 6 7 0 1 2
#       ^ ^
# 6 7 0 1 2 4 5
#   ^ ^


class Solution:
    def findMin(self, nums: list[int]) -> int:
        l, r = 0, len(nums) - 1
        
        if nums[l] < nums[r]:
            return nums[l]
        
        while r - l > 1:
            m = (l + r) // 2
            
            if nums[m] > nums[r]:
                l = m
            
            else:
                # nums[m] < nums[r]
                r = m
                
        return nums[r]