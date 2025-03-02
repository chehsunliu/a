class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        l, r = 0, len(matrix) - 1

        while l <= r:
            m = (l + r) // 2

            if target == matrix[m][0]:
                return True
            elif target > matrix[m][0]:
                l = m + 1
            else:
                r = m - 1

        if r < 0:
            return False

        offset = r
        l, r = 0, len(matrix[offset]) - 1

        while l <= r:
            m = (l + r) // 2

            if target == matrix[offset][m]:
                return True
            elif target > matrix[offset][m]:
                l = m + 1
            else:
                r = m - 1

        return False

# 35
# 11 22 33 33 44
#          r  l