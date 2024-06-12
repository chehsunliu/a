# 1 2 2 2 3 3 3
# l     m     r
# l m r
def search_l(nums: list[int], target: int) -> list[int]:
    l, r = 0, len(nums) - 1

    while l < r:
        m = (l + r) // 2

        if nums[m] == target:
            r = m
        
        elif nums[m] < target:
            l = m + 1
        
        else:
            r = m - 1
            
    return r

            
    
# 0 1 1 1 1 2 2
# l     m     r
#         l m r
#         l r
def search_r(nums: list[int], target: int) -> list[int]:
    l, r = 0, len(nums) - 1

    while l < r:
        m = (l + r) // 2

        if nums[m] == target:
            l = m + 1

        elif nums[m] < target:
            l = m + 1
            
        else:
            r = m
            
    return l


class Solution:
    def searchRange(self, nums: list[int], target: int) -> list[int]:
        if not nums:
            return [-1, -1]

        l = search_l(nums, target)
        r = search_r(nums, target)
        
        if nums[l] != target:
            return [-1, -1]

        if nums[r] != target:
            return [l, r - 1]
        else:
            return [l, r]