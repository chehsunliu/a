class Solution:
    def maxSubArray(self, nums: list[int]) -> int:
        best_ans = nums[0]
        
        for i in range(len(nums)):
            ans = nums[i]
            best_ans = max(best_ans, ans)
            for j in range(i + 1, len(nums)):
                ans += nums[j]
                best_ans = max(best_ans, ans)
            
        return best_ans
