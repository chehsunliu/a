class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        for i in range(3):
            for j in range(3):
                if not self.is_valid_block(board, i * 3, j * 3):
                    return False

        for i in range(9):
            if not self.is_valid_row(board, i):
                return False

        for j in range(9):
            if not self.is_valid_col(board, j):
                return False

        return True

    def is_valid_block(self, board: list[list[str]], i: int, j: int) -> bool:
        vs = {}
        for offset_i in range(3):
            for offset_j in range(3):
                c = board[i + offset_i][j + offset_j]
                if c == ".":
                    continue
                v = int(c)
                vs[v] = vs.get(v, 0) + 1
        return check(vs)

    def is_valid_row(self, board: list[list[str]], i: int) -> bool:
        vs = {}
        for j in range(9):
            c = board[i][j]
            if c == ".":
                continue
            v = int(c)
            vs[v] = vs.get(v, 0) + 1
        return check(vs)

    def is_valid_col(self, board: list[list[str]], j: int) -> bool:
        vs = {}
        for i in range(9):
            c = board[i][j]
            if c == ".":
                continue
            v = int(c)
            vs[v] = vs.get(v, 0) + 1
        return check(vs)


def check(vs: dict[int, int]) -> bool:
    for v in vs.values():
        if v > 1:
            return False
    return True
