class Solution:
    def arrayPairSum(self, nums: list[int]) -> int:
        return sum([v for i, v in enumerate(sorted(nums)) if i % 2 == 0])