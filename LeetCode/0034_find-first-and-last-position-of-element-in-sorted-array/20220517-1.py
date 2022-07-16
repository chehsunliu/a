# 0 1 2 3 4 5
# 5,6,7,7,9,10 - 7


def search_left(nums: list[int], target: int) -> int:
    l, r = 0, len(nums) - 1
    
    while l <= r:
        m = l + (r - l) // 2
        
        if nums[m] < target:
            l = m + 1
        
        else:  # nums[m] >= target
            r = m - 1
            
    return l


def search_right(nums: list[int], target: int) -> int:
    l, r = 0, len(nums) - 1
    
    while l <= r:
        m = l + (r - l) // 2
        
        if nums[m] <= target:
            l = m + 1
        
        else:  # nums[m] > target
            r = m - 1
            
    return l


class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        l = search_left(nums, target)
        r = search_right(nums, target) - 1

        if not nums or l >= len(nums) or nums[l] != target:
            return [-1, -1]
        
        return [l, r]