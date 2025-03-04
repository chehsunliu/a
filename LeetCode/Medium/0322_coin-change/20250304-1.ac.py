class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        v = f(coins, amount, {})
        if v is not None:
            return v
        else:
            return -1


def f(coins: list[int], amount: int, dp: dict[int, int | None]):
    if amount == 0:
        return 0

    if amount < 0:
        return None

    if amount in dp:
        return dp[amount]

    ans = None
    for coin in coins:
        v = f(coins, amount - coin, dp)
        if v is not None:
            if ans is not None:
                ans = min(ans, 1 + v)
            else:
                ans = 1 + v

    dp[amount] = ans
    return ans
