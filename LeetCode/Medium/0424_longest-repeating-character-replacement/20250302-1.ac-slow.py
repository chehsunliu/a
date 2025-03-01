class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        freq = {chr(ord("A") + offset): 0 for offset in range(26)}

        ans = 0
        l, r = 0, 0
        while r < len(s):
            window_size = r - l + 1
            freq[s[r]] += 1
            max_freq = max(freq.values())

            if window_size - max_freq <= k:
                ans = max(ans, window_size)
            else:
                while window_size - max_freq > k:
                    freq[s[l]] -= 1
                    l += 1
                    window_size -= 1
                    max_freq = max(freq.values())

            r += 1
        return ans
