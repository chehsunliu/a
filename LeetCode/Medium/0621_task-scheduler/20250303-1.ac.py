class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        freqs = {}
        for t in tasks:
            if t not in freqs:
                freqs[t] = 0
            freqs[t] += 1

        h = []
        for t, freq in freqs.items():
            heapq.heappush(h, (-freq, t))

        ans = 0
        while len(h) > 0:
            tmp = []
            count = 0
            for _ in range(n + 1):
                if len(h) == 0:
                    break

                count += 1
                freq, t = heapq.heappop(h)
                freq = -freq

                freq -= 1
                if freq > 0:
                    tmp.append((-freq, t))

            for item in tmp:
                heapq.heappush(h, item)

            if len(h) > 0:
                ans += n + 1
            else:
                ans += count

        return ans