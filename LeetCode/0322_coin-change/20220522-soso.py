class Solution:
    def coinChange(self, coins: list[int], amount: int) -> int:
        self.coins = coins
        self.dp = {0: 0}
        
        answer = self.f(amount)
        return answer if answer is not None else -1
        
    def f(self, target: int):
        if target in self.dp:
            return self.dp[target]
        
        if target < 0:
            return None
        
        results = []
        for coin in self.coins:
            result = self.f(target - coin)
            if result is not None:
                results.append(result)
        
        answer = min(results) + 1 if results else None
        self.dp[target] = answer
        return answer
