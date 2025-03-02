class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        l, r = 1, max(piles)

        while l <= r:
            m = (l + r) // 2
            h_actual = self.f(piles, m)

            if h_actual <= h:
                r = m - 1
            else:
                l = m + 1

        return l

    def f(self, piles: list[int], k: int) -> int:
        return sum([math.ceil(p / k) for p in piles])

# l=1 r=30
# 31/2=15
# 46/2=23
# 54/2=27
# 58/2=29
# 60/2=30