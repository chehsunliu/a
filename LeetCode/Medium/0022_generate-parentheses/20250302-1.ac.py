dp = {}
count = 0

class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        ans = list(g(n))
        print(count)
        return ans


def g(n: int) -> list[str]:
    global count

    count += 1
    if n == 1:
        return set(["()"])

    if n in dp:
        return dp[n]

    ans = set()
    for i in range(1, n):
        g1 = g(i)
        g2 = g(n - i)
        for gg1 in g1:
            for gg2 in g2:
                ans.add(gg1 + gg2)
    g3 = g(n - 1)
    for gg in g3:
        ans.add("(" + gg + ")")

    dp[n] = ans
    return ans
