class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        target = sum(nums)
        if target % 2 == 1:
            return False

        target = target // 2

        return f(nums, 0, target)


def f(nums: list[int], index: int, target: int) -> bool:
    if index >= len(nums):
        return False

    if target == 0:
        return True

    return f(nums, index + 1, target) or f(nums, index + 1, target - nums[index])
