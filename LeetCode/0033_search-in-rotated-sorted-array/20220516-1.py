def find_pivot(nums: list[int]) -> int:
    l, r = 0, len(nums) - 1
    
    while True:
        m = l + (r - l) // 2

        ascending_l = nums[l] <= nums[m]
        ascending_r = nums[m + 1] <= nums[r]
        
        if ascending_l and ascending_r:
            break
            
        elif not ascending_l:
            r = m
        
        else:
            l = m + 1
    
    return m


def binary_search_target(nums: list[int], l: int, r: int, target: int) -> int:
    while l <= r:
        m = l + (r - l) // 2
        
        if nums[m] == target:
            return m
        
        elif nums[m] < target:
            l = m + 1
        
        else:
            r = m - 1
    
    return -1


class Solution:
    def search(self, nums: list[int], target: int) -> int:
        if len(nums) == 1 or nums[0] < nums[len(nums) - 1]:
            return binary_search_target(nums, l=0, r=len(nums) - 1, target=target)
        
        pivot = find_pivot(nums)
        
        index_l = binary_search_target(nums, l=0, r=pivot, target=target)
        if index_l >= 0:
            return index_l

        return binary_search_target(nums, l=pivot + 1, r=len(nums) - 1, target=target)
