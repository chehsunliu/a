class Solution:
    def canJump(self, nums: list[int]) -> bool:
        current_index = 0
        
        while True:
            next_index = current_index
            max_reachable = current_index

            for index in range(current_index + 1, min(current_index + nums[current_index] + 1, len(nums))):
                reachable = index + nums[index]
                
                if reachable >= max_reachable:
                    max_reachable = reachable
                    next_index = index
                    
            if next_index >= len(nums) - 1:
                return True
            elif nums[next_index] == 0:
                return False
            
            current_index = next_index