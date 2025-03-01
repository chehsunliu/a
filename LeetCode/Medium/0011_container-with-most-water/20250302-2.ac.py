class Solution:
    def maxArea(self, height: List[int]) -> int:
        ans = 0
        l, r = 0, len(height) - 1

        while l < r:
            ans_l = (r - l) * min(height[l], height[r])
            ans = max(ans_l, ans)

            if height[l] == height[r]:
                l += 1
                r -= 1
            elif height[l] > height[r]:
                r -= 1
            else:
                l += 1

        return ans