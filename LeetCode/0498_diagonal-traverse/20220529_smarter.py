class Solution:
    def findDiagonalOrder(self, mat: list[list[int]]) -> list[int]:
        m = len(mat)
        n = len(mat[0])

        diagonals = [[] for _ in range(m + n - 1)]
        
        for i in range(m):
            for j in range(n):
                diagonals[i + j].append(mat[i][j])
                
        answer = []
        
        for i, diagonal in enumerate(diagonals):
            if i % 2 == 0:
                answer += diagonal[::-1]
            else:
                answer += diagonal
                
        return answer
                