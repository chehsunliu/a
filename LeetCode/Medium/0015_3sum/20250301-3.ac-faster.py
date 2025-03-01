class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        counts = {}
        new_nums = []
        for v in nums:
            if v not in counts:
                counts[v] = 0
            elif counts[v] >= 3:
                continue

            new_nums.append(v)
            counts[v] += 1

        nums = sorted(new_nums)

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

        return list(ans)