class Solution:
    def trap(self, height: list[int]) -> int:
        area = 0
        max_h = 0
        l_highest_x = r_highest_x = 0

        for x, h in enumerate(height):
            if h > max_h:
                max_h = h
                l_highest_x = r_highest_x = x
            
            elif h == max_h:
                r_highest_x = x
                
        prev_x = 0
        prev_h = height[prev_x]
        for x in range(prev_x + 1, l_highest_x + 1):
            if height[x] < prev_h:
                area -= height[x]
            else:
                area += (x - prev_x - 1) * prev_h
                prev_x = x
                prev_h = height[x]

        prev_x = len(height) - 1
        prev_h = height[prev_x]
        for x in range(prev_x - 1, r_highest_x - 1, -1):
            if height[x] < prev_h:
                area -= height[x]
            else:
                area += (prev_x - x - 1) * prev_h
                prev_x = x
                prev_h = height[x]
                
        area += max(0, r_highest_x - l_highest_x - 1) * max_h
        for x in range(l_highest_x + 1, r_highest_x):
            area -= height[x]
                
        return area
