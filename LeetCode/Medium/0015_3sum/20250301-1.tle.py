class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        targets = {}
        for i, v in enumerate(nums):
            if v not in targets:
                targets[v] = set()
            targets[v].add(i)

        ans = set()
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                tmp = nums[i] + nums[j]
                if -tmp not in targets:
                    continue
                ks = targets[-tmp]

                for k in ks:
                    if k <= j:
                        continue

                    a = tuple(sorted([nums[i], nums[j], nums[k]]))
                    ans.add(a)

        return list(ans)