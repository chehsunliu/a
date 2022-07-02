class Solution:
    def findDiagonalOrder(self, mat: list[list[int]]) -> list[int]:
        answer = []
        
        m = len(mat)
        n = len(mat[0])
        
        move_upward = True
        i, j = 0, 0
        
        while len(answer) < m * n:
            answer.append(mat[i][j])

            if move_upward:
                if j + 1 >= n:
                    i, j = i + 1, j
                    move_upward = False
                elif i - 1 < 0:
                    i, j = i, j + 1
                    move_upward = False
                else:
                    i, j = i - 1, j + 1
            else:
                if i + 1 >= m:
                    i, j = i, j + 1
                    move_upward = True
                elif j - 1 < 0:
                    i, j = i + 1, j
                    move_upward = True
                else:
                    i, j = i + 1, j - 1
                    
        return answer
