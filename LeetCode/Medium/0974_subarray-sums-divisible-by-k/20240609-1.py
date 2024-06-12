class Solution:
    def subarraysDivByK(self, nums: List[int], k: int) -> int:
        rs = [0 for _ in range(len(nums) + 1)]

        for i in range(len(nums) - 1, -1, -1):
            rs[i] = (rs[i + 1] + nums[i]) % k

        counts = {}

        for i in range(len(rs) - 1):
            r = rs[i]
            counts[r] = counts.get(r, 0) + 1

        ans = 0

        for r, count in counts.items():
            if r > 0:
                ans += count * (count - 1) // 2
            else:
                ans += count * (count - 1) // 2 + count

        return ans
