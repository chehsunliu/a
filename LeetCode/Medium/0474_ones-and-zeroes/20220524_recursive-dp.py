class Counter:
    def __init__(self, s: str) -> None:
        result = {"0": 0, "1": 0}
        for c in s:
            result[c] += 1

        self.zeros = result["0"]
        self.ones = result["1"]


class Solution:
    def findMaxForm(self, strs: list[str], m: int, n: int) -> int:
        self.counters: list[Counter] = [Counter(s) for s in strs]
        self.dp = [[[None for _ in range(n + 1)] for _ in range(m + 1)] for _ in range(len(self.counters) + 1)]
        return self.f(0, m, n)
        
    def f(self, index: int, zeros: int, ones: int):
        if self.dp[index][zeros][ones] is not None:
            return self.dp[index][zeros][ones]
        
        if index >= len(self.counters):
            return 0
        
        c = self.counters[index]
        
        if zeros >= c.zeros and ones >= c.ones:
            ans0 = self.f(index + 1, zeros, ones)
            ans1 = 1 + self.f(index + 1, zeros - c.zeros, ones - c.ones)
            self.dp[index][zeros][ones] = max(ans0, ans1)
        else:
            self.dp[index][zeros][ones] = self.f(index + 1, zeros, ones)

        return self.dp[index][zeros][ones]