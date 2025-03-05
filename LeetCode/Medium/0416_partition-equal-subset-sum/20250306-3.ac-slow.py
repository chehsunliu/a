class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        target = sum(nums)
        if target % 2 == 1:
            return False

        target = target // 2

        dp = {}

        return f(nums, 0, target, dp)


def f(nums: list[int], index: int, target: int, dp) -> bool:
    if index >= len(nums):
        return False

    if target == 0:
        return True

    if (index, target) in dp:
        return dp[(index, target)]

    result = f(nums, index + 1, target - nums[index], dp) or f(nums, index + 1, target, dp)
    dp[(index, target)] = result
    return result
