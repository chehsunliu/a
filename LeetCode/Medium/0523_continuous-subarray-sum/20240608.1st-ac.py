class Solution:
    def checkSubarraySum(self, nums: List[int], k: int) -> bool:
        sums = [0 for _ in range(len(nums) + 1)]
        rs = {}

        sums[-1] = 0
        for i in range(len(sums) - 2, -1, -1):
            sums[i] = nums[i] + sums[i + 1]
            
            r = sums[i] % k
            if r not in rs:
                rs[r] = []
            rs[r].append(i)

        for r, indexes in rs.items():
            if r == 0:
                if indexes[-1] != len(nums) - 1:
                    return True
                continue

            if len(indexes) > 2:
                return True

            if len(indexes) == 1:
                continue

            if indexes[0] - indexes[1] > 1:
                return True

        return False
