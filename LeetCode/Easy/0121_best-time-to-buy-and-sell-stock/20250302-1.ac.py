class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        ans = 0

        h = prices[-1]
        for i in range(len(prices) - 1, -1, -1):
            ans = max(h - prices[i], ans)
            h = max(prices[i], h)

        return ans