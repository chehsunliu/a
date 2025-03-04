class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        v = f(coins, amount, {0: 0})
        return v if v is not None else -1


def f(coins: list[int], amount: int, dp: dict[int, int | None]):
    if amount < 0:
        return None

    if amount in dp:
        return dp[amount]

    results = []
    for coin in coins:
        v = f(coins, amount - coin, dp)
        if v is not None:
            results.append(v)

    ans = min(results) + 1 if results else None
    dp[amount] = ans
    return ans
