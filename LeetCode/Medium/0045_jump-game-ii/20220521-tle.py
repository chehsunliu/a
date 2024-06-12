from collections import deque


class Solution:
    def jump(self, nums: list[int]) -> int:
        dp = [sys.maxsize for _ in nums]
        dp[0] = 0

        queue = deque()
        queue.append(0)

        while queue:
            i = queue.popleft()
            
            for distance in range(1, nums[i] + 1):
                next_i = i + distance
                if next_i >= len(nums):
                    break

                if dp[i] + 1 < dp[next_i]:
                    dp[next_i] = dp[i] + 1
                    queue.append(next_i)
                    
                else:
                    pass
                
        return dp[len(nums) - 1]