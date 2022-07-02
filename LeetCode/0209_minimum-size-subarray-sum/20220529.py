class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        l, r = 0, 0
        current_sum = nums[0]
        answer = None
        
        while r < len(nums):
            if l > r:
                r = l
                if r >= len(nums):
                    break
                current_sum = nums[r]
            
            if current_sum < target:
                r += 1
                if r < len(nums):
                    current_sum += nums[r]
            
            else:
                answer = min(answer, r - l + 1) if answer is not None else (r - l + 1)
                
                current_sum -= nums[l]
                l += 1
                
        return answer if answer is not None else 0