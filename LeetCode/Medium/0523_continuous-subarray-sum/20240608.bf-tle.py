class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        sums = [0 for _ in range(len(nums) + 1)]

        sums[-1] = 0
        for i in range(len(sums) - 2, -1, -1):
            sums[i] = nums[i] + sums[i + 1]

        for i in range(len(nums) - 1):
            for j in range(i + 1, len(nums)):
                v = sums[i] - sums[j + 1]
                if v % k == 0:
                    return True

        return False
