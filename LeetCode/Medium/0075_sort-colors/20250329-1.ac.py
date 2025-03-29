class Solution:
    def sortColors(self, nums: List[int]) -> None:
        c = {}
        for v in nums:
            if v not in c:
                c[v] = 0
            c[v] += 1

        i = 0
        for v in range(3):
            count = c.get(v, 0)
            for cc in range(count):
                nums[i] = v
                i += 1
