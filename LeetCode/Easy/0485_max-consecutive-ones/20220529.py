class Solution:
    def findMaxConsecutiveOnes(self, nums: list[int]) -> int:
        answer = 0
        base, i = -1, 0

        while i < len(nums):
            if nums[i] == 0:
                answer = max(answer, i - base - 1)
                base = i
            
            i += 1
            
        return max(answer, len(nums) - base - 1)
