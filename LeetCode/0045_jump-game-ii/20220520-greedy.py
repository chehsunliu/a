from collections import deque


class Solution:
    def jump(self, nums: list[int]) -> int:
        if len(nums) == 1:
            return 0

        steps = 1
        current_pos = 0
        
        while True:
            max_pos = 0
            
            for d in range(1, nums[current_pos] + 1):
                possible_pos = current_pos + d
                if possible_pos >= len(nums) - 1:
                    return steps
                
                if possible_pos + nums[possible_pos] >= max_pos:
                    next_pos = possible_pos
                    max_pos = possible_pos + nums[possible_pos]

            current_pos = next_pos
            steps += 1
