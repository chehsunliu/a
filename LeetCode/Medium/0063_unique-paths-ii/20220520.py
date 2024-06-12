class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: list[list[int]]) -> int:
        m = len(obstacleGrid)
        n = len(obstacleGrid[0])
        
        ways = [[0 for _ in range(n)] for _ in range(m)]
        ways[m - 1][n - 1] = 1
        
        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                if obstacleGrid[i][j]:
                    ways[i][j] = 0
                
                else:
                    if j + 1 < n:
                        ways[i][j] += ways[i][j + 1]
                    if i + 1 < m:
                        ways[i][j] += ways[i + 1][j]
        
        return ways[0][0]