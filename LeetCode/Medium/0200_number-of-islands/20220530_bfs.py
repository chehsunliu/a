class Solution:
    def numIslands(self, grid: list[list[str]]) -> int:
        m = len(grid)
        n = len(grid[0])
        
        labels = [[None for _ in range(n)] for _ in range(m)]
        label = 0
        
        points = []
        for i in range(m):
            for j in range(n):
                if grid[i][j] == "1":
                    points.append((i, j))
                    
        for s_i, s_j in points:
            if labels[s_i][s_j] is not None:
                continue

            queue = deque()
            queue.append((s_i, s_j))
            
            while queue:
                i, j = queue.popleft()
                if i < 0 or i >= m or j < 0 or j >= n or labels[i][j] is not None or grid[i][j] == "0":
                    continue
                    
                labels[i][j] = label
                queue.append((i + 1, j))
                queue.append((i, j + 1))
                queue.append((i - 1, j))
                queue.append((i, j - 1))
            
            label += 1
            
        return label