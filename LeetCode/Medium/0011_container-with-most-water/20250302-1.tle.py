class Solution:
    def maxArea(self, height: List[int]) -> int:
        ans = 0
        for i in range(len(height)):
            for j in range(i + 1, len(height)):
                ans = max(ans, (j - i) * min(height[i], height[j]))
        return ans