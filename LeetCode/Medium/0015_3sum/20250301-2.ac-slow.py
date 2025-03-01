class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums = sorted(nums)

        ans = set()
        for k, target in enumerate(nums):
            l, r = 0, k - 1

            while l < r:
                if nums[l] + nums[r] == -target:
                    ans.add(tuple(sorted((nums[l], nums[r], target))))
                    r -= 1
                    l += 1

                elif nums[l] + nums[r] > -target:
                    r -= 1

                else:
                    l += 1

        return list([list(a) for a in ans])