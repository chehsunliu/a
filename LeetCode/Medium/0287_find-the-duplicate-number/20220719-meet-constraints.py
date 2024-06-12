def count(nums: list[int], x: int) -> int:
    return sum([num <= x for num in nums])


class Solution:
    def findDuplicate(self, nums: list[int]) -> int:
        l, r = 1, len(nums)
        
        while l <= r:
            m = (l + r) // 2
            
            if count(nums, m) <= m:
                l = m + 1
            else:
                r = m - 1
                
        return l
            
            
# 1 2 5 5 5
#   r l