class NumMatrix:

    def __init__(self, matrix: list[list[int]]):
        m = len(matrix)
        n = len(matrix[0])
        self.sums = [[0 for _ in range(n)] for _ in range(m)]
        
        for i in range(m):
            for j in range(n):
                # A B
                # C D

                sum_a = self.sums[i - 1][j - 1] if i >= 1 and j >= 1 else 0
                sum_b = self.sums[i - 1][j] if i >= 1 else 0
                sum_c = self.sums[i][j - 1] if j >= 1 else 0
                
                self.sums[i][j] = matrix[i][j] + sum_b + sum_c - sum_a

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        sum_a = self.sums[row1 - 1][col1 - 1] if row1 >= 1 and col1 >= 1 else 0
        sum_b = self.sums[row1 - 1][col2] if row1 >= 1 else 0
        sum_c = self.sums[row2][col1 - 1] if col1 >= 1 else 0
        return self.sums[row2][col2] - sum_b - sum_c + sum_a
        


# Your NumMatrix object will be instantiated and called as such:
# obj = NumMatrix(matrix)
# param_1 = obj.sumRegion(row1,col1,row2,col2)