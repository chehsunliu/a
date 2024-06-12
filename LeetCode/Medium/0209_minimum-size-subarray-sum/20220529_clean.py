class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        answer = len(nums) + 1
        l = 0

        current_sum = 0
        for i in range(len(nums)):
            current_sum += nums[i]
            
            while current_sum >= target:
                answer = min(answer, i - l + 1)
                current_sum -= nums[l]
                l += 1
        
        return answer if answer != len(nums) + 1 else 0