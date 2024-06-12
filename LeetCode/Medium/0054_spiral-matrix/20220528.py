class Solution:
    def spiralOrder(self, matrix: list[list[int]]) -> list[int]:
        i, j = 0, 0
        ans = [matrix[i][j]]
        
        m, n = len(matrix), len(matrix[0])

        visited = [[False for _ in range(n)] for _ in range(m)]
        visited[i][j] = True
        
        d = deque([(-1, 0), (0, -1), (1, 0), (0, 1)])

        while len(ans) < m * n:
            offset_i, offset_j = d[-1]
            next_i, next_j = i + offset_i, j + offset_j
            
            if (0 <= next_i < m) and (0 <= next_j < n) and not visited[next_i][next_j]:
                ans.append(matrix[next_i][next_j])
                visited[next_i][next_j] = True
                i, j = next_i, next_j
            else:
                d.rotate(1)
            
        return ans
