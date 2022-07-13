class Solution:
    def floodFill(self, image: list[list[int]], sr: int, sc: int, color: int) -> list[list[int]]:
        self.image = image
        self.m = len(image)
        self.n = len(image[0])

        self.color = color
        self.original_color = image[sr][sc]
        if self.color == self.original_color:
            return self.image
        
        self.f(sr, sc)
        return self.image
        
    def f(self, r: int, c: int):
        if r < 0 or r >= self.m or c < 0 or c >= self.n:
            return
        
        if self.image[r][c] != self.original_color:
            return
        
        self.image[r][c] = self.color
        
        self.f(r + 1, c)
        self.f(r, c + 1)
        self.f(r - 1, c)
        self.f(r, c - 1)
        