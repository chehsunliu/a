class Solution:
    def updateMatrix(self, mat: list[list[int]]) -> list[list[int]]:
        m = len(mat)
        n = len(mat[0])
        
        queue = deque()

        for i in range(m):
            for j in range(n):
                if mat[i][j] != 0:
                    mat[i][j] = sys.maxsize
                else:
                    queue.append((i, j))
        
        while queue:
            i0, j0 = queue.popleft()
            value = mat[i0][j0] + 1
            
            if i0 + 1 < m and value < mat[i0 + 1][j0]:
                mat[i0 + 1][j0] = value
                queue.append((i0 + 1, j0))
            
            if i0 - 1 >= 0 and value < mat[i0 - 1][j0]:
                mat[i0 - 1][j0] = value
                queue.append((i0 - 1, j0))
            
            if j0 + 1 < n and value < mat[i0][j0 + 1]:
                mat[i0][j0 + 1] = value
                queue.append((i0, j0 + 1))
            
            if j0 - 1 >= 0 and value < mat[i0][j0 - 1]:
                mat[i0][j0 - 1] = value
                queue.append((i0, j0 - 1))
                
        return mat