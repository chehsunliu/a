class Solution:
    def validPalindrome(self, s: str) -> bool:
        j = len(s) // 2
        if f(s, j if len(s) % 2 == 1 else j - 1, j):
            return True

        for i in range(len(s)):
            s2 = s[:i] + s[i + 1:]
            j2 = len(s2) // 2
            if f(s2, j2 if len(s2) % 2 == 1 else j2 - 1, j2):
                return True

        return False


def f(s: str, i: int, j: int) -> bool:
    while i >= 0 and j < len(s):
        if s[i] != s[j]:
            return False
        i -= 1
        j += 1
    return True

# 5 // 2 = 2, 0 1 2 3 4
# 1 // 2 = 0, 0

# 4 // 2 = 2, 0 1 2 3
# 2 // 2 = 1, 0 1