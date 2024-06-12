class Board:
    def __init__(self, n: int) -> None:
        self.n = n
        self.data = [[None for _ in range(n)] for _ in range(n)]
        
    def __getitem__(self, key: tuple[int, int]) -> Optional[str]:
        i, j = key
        return self.data[i][j]
    
    def __setitem__(self, key: tuple[int, int], value: Optional[str]) -> None:
        i, j = key
        if i < 0 or i >= self.n or j < 0 or j >= self.n:
            return

        self.data[i][j] = value
        
    def copy(self) -> "Board":
        result = Board(n=self.n)
        for i in range(self.n):
            for j in range(self.n):
                result[i, j] = self[i, j]
        return result
    
    def __str__(self) -> str:
        return self.__repr__()

    def __repr__(self) -> str:
        s = ",".join(["\"" + "".join([(str(v) if v else "_") for v in self.data[i]]) + "\"" for i in range(self.n)])
        return "[" + s + "]"
    

class Solution:
    def solveNQueens(self, n: int) -> list[list[str]]:
        self.answer = set()
        
        board = Board(n)
        for i in range((n + 1) // 2):
            for j in range((n + 1) // 2):
                self.f(board.copy(), i, j, 1)
                
        return [list(v) for v in self.answer]
                
    def f(self, board: Board, i: int, j: int, count: int):
        for c in range(board.n):
            board[i, c] = "."
            
        for r in range(board.n):
            board[r, j] = "."
            
        for offset in range(1, board.n):
            board[i - offset, j - offset] = "."
            board[i - offset, j + offset] = "."
            board[i + offset, j - offset] = "."
            board[i + offset, j + offset] = "."
            
        board[i, j] = "Q"
        if count == board.n:
            self.answer.add(tuple("".join(board.data[r]) for r in range(board.n)))
            return

        for r in range(board.n):
            for c in range(board.n):
                if board[r, c] is None:
                    self.f(board.copy(), r, c, count + 1)