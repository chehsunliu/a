# 1 1 2 2 3 3 3 3
# l     m       r
# l m r
#     m
#   r l

def search_l(nums: list[int], target: int) -> list[int]:
    l, r = 0, len(nums) - 1

    while l <= r:
        m = (l + r) // 2

        if nums[m] >= target:
            r = m - 1
        else:
            l = m + 1
            
    return l


class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        if not nums:
            return [-1, -1]
        
        l = search_l(nums, target)
        r = search_l(nums, target + 1)
        
        if l >= len(nums) or nums[l] != target:
            return [-1, -1]

        return [l, r - 1]