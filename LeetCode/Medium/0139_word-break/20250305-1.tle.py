class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        return f(s, set(wordDict))


def f(s: str, words: set[str]) -> bool:
    if len(s) == 0:
        return True

    result = False
    for l in range(1, len(s) + 1):
        ss = s[:l]
        result = result or (ss in words and f(s[l:], words))
    return result
