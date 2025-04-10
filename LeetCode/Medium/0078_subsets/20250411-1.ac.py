class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        ans = []
        f(nums, 0, [], ans)
        return ans


def f(nums: list[int], i: int, v: list[int], vs: list[list[int]]):
    if i >= len(nums):
        vs.append(v)
        return

    f(nums, i + 1, v, vs)

    v = v.copy()
    v.append(nums[i])
    f(nums, i + 1, v, vs)