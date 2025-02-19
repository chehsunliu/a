class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        ps1 = [1] * len(nums)
        for i in range(1, len(nums)):
            ps1[i] = ps1[i - 1] * nums[i - 1]

        ps2 = [1] * len(nums)
        for i in range(len(nums) - 2, -1, -1):
            ps2[i] = ps2[i + 1] * nums[i + 1]

        return [ps1[i] * ps2[i] for i in range(len(nums))]
