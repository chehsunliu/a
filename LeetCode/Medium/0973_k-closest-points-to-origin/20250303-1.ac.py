class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []

        for p in points:
            v = p[0] * p[0] + p[1] * p[1]
            heapq.heappush(heap, (v, p))

        ans = []
        for i in range(k):
            ans.append(heapq.heappop(heap)[1])

        return ans