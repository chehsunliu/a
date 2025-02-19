class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        outs = [1] * len(nums)

        p1, p2 = 1, 1
        for i in range(len(nums)):
            outs[i] *= p1
            p1 *= nums[i]

            j = len(nums) - i - 1
            outs[j] *= p2
            p2 *= nums[j]

        return outs
