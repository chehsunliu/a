class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        acc = set()
        ans = 0

        l, r = 0, 0
        while r < len(s):
            while s[r] in acc:
                acc.remove(s[l])
                l += 1
            acc.add(s[r])
            ans = max(ans, len(acc))

            r += 1

        return ans