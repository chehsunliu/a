class Solution:
    def topKFrequent(self, nums: list[int], k: int) -> list[int]:
        counts = {}
        for n in nums:
            counts[n] = counts.get(n, 0) + 1

        arr = [(count, n) for n, count in counts.items()]
        arr = list(sorted(arr, key=lambda x: x[0], reverse=True))
        return [v[1] for v in arr[:k]]
