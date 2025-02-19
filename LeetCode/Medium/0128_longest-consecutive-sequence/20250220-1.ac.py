class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        m = {}
        for num in nums:
            m[num] = None

        ans = 0
        for num in m:
            ans = max(ans, f(m, num))
        return ans


def f(m, num):
    if num not in m:
        return 0

    v = m[num]
    if v is not None:
        return v

    m[num] = 1 + f(m, num + 1)
    return m[num]


# 4 3 1 2 5
# 2 3     1