class Solution:
    def rotate(self, matrix: list[list[int]]) -> None:
        bound0 = 0
        bound1 = len(matrix) - 1
        
        while bound0 < bound1:
            pivots = [
                [bound0, bound0],
                [bound0, bound1],
                [bound1, bound1],
                [bound1, bound0],
            ]
            for i in range(bound1 - bound0):
                tmp = matrix[pivots[3][0]][pivots[3][1]]
                for k in range(3, 0, -1):
                    matrix[pivots[k][0]][pivots[k][1]] = matrix[pivots[k - 1][0]][pivots[k - 1][1]]
                matrix[pivots[0][0]][pivots[0][1]] = tmp

                pivots[0][1] += 1
                pivots[1][0] += 1
                pivots[2][1] -= 1
                pivots[3][0] -= 1

            bound0 += 1
            bound1 -= 1