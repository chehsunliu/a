class Solution:
    def wallsAndGates(self, rooms: list[list[int]]) -> None:
        self.rooms = rooms

        self.m = len(rooms)
        self.n = len(rooms[0])
        
        self.directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]

        for i in range(self.m):
            for j in range(self.n):
                if rooms[i][j] == 0:
                    for offset_i, offset_j in self.directions:
                        self.traverse(i + offset_i, j + offset_j, 1)

    def traverse(self, i: int, j: int, steps: int) -> None:
        if i < 0 or i >= self.m or j < 0 or j >= self.n:
            return
        
        if self.rooms[i][j] <= steps:
            return

        self.rooms[i][j] = steps
        
        for offset_i, offset_j in self.directions:
            self.traverse(i + offset_i, j + offset_j, steps + 1)