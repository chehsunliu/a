class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        h = []

        for w in stones:
            heapq.heappush(h, -w)

        while len(h) >= 2:
            w1 = -heapq.heappop(h)
            w2 = -heapq.heappop(h)

            if w1 != w2:
                heapq.heappush(h, -(w1 - w2))

        if len(h) == 1:
            return -h[0]
        else:
            return 0