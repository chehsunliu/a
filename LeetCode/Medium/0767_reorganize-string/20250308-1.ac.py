class Solution:
    def reorganizeString(self, s: str) -> str:
        freq = {}
        for c in s:
            if c not in freq:
                freq[c] = 0
            freq[c] += 1

        h = []
        for c, count in freq.items():
            heapq.heappush(h, (-count, c))

        ans = ""
        h_tmp = None
        while len(h) > 0:
            minus_count, c = heapq.heappop(h)
            count = -minus_count

            if len(ans) > 0 and ans[-1] == c:
                assert h_tmp is None
                h_tmp = -count, c
            else:
                ans += c
                count -= 1
                if count > 0:
                    heapq.heappush(h, (-count, c))
                if h_tmp is not None:
                    heapq.heappush(h, h_tmp)
                    h_tmp = None

        if h_tmp is not None:
            return ""

        return ans