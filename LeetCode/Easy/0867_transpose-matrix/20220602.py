class Solution:
    def transpose(self, matrix: list[list[int]]) -> list[list[int]]:
        m = len(matrix)
        n = len(matrix[0])
        result = [[None for _ in range(m)] for _ in range(n)]
        
        for i in range(m):
            for j in range(n):
                result[j][i] = matrix[i][j]
                
        return result