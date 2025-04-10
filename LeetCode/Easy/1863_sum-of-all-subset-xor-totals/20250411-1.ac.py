class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        return f(nums, 0, 0)


def f(nums: list[int], i: int, v: int):
    if i >= len(nums):
        return v

    a1 = f(nums, i + 1, v ^ nums[i])
    a2 = f(nums, i + 1, v ^ 0)
    return a1 + a2