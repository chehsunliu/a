class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        self.m = len(grid)
        self.n = len(grid[0])
        
        self.grid = grid
        self.labels = [[None for _ in range(self.n)] for _ in range(self.m)]
        
        count = 0
        for i in range(self.m):
            for j in range(self.n):
                count += self.f(i, j)
        return count
                
    def f(self, i: int, j: int) -> 0:
        if i < 0 or i >= self.m or j < 0 or j >= self.n:
            return 0
        
        if self.grid[i][j] == "0" or self.labels[i][j] is not None:
            return 0
        
        self.labels[i][j] = True

        self.f(i + 1, j)
        self.f(i, j + 1)
        self.f(i - 1, j)
        self.f(i, j - 1)
        
        return 1