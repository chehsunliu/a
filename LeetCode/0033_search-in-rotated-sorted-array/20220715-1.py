class Solution:
    def search(self, nums: list[int], target: int) -> int:
        offset = self.find_pivot(nums)
        
        l, r = 0, len(nums) - 1
        
        while l <= r:
            m = (l + r) // 2
            m_ = (m + offset) % len(nums)
            
            if nums[m_] == target:
                return m_
            elif nums[m_] < target:
                l = m + 1
            else:
                r = m - 1
                
        return -1
    
    def find_pivot(self, nums: list[int]) -> int:
        l, r = 0, len(nums) - 1
        
        if nums[l] < nums[r]:
            return 0
        
        while r - l > 1:
            m = (l + r) // 2
            
            if nums[l] > nums[m]:
                r = m
            
            else:
                l = m
                
        return r
