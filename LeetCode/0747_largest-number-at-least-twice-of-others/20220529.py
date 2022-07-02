class Solution:
    def dominantIndex(self, nums: list[int]) -> int:
        maximum = 0
        index = 0
        
        for i, num in enumerate(nums):
            if num >= maximum:
                maximum = num
                index = i
                
        for i, num in enumerate(nums):
            if i == index:
                continue

            if num * 2 > maximum:
                return -1
            
        return index