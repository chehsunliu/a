class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        return f(s, set(wordDict), dp={})


def f(s: str, words: set[str], dp: dict[str, bool]) -> bool:
    if len(s) == 0:
        return True

    if s in dp:
        return dp[s]

    result = False
    for l in reversed(range(1, len(s) + 1)):
        ss = s[:l]
        if ss in words and f(s[l:], words, dp=dp):
            result = True

    dp[s] = result
    return result
