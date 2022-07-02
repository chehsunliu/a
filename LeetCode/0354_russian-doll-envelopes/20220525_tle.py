class Item:
    def __init__(self, w: int, h: int):
        self.w = w
        self.h = h


class Solution:
    def maxEnvelopes(self, envelopes: list[list[int]]) -> int:
        self.items = [Item(w=e[0], h=e[1]) for e in envelopes]
        self.items = sorted(self.items, key=lambda x: (x.w, x.h), reverse=True)
        self.dp = [None for _ in self.items]
        
        ans = 0
        for i in range(len(self.items)):
            ans = max(ans, self.f(i))
            
        return ans
        
    def f(self, index: int) -> int:
        if index >= len(self.items):
            return 0
        
        if self.dp[index]:
            return self.dp[index]

        current_item = self.items[index]
        
        max_layers = 0
        for i in range(index + 1, len(self.items)):
            item = self.items[i]
            if item.w < current_item.w and item.h < current_item.h:
                max_layers = max(max_layers, self.f(i))
                
        self.dp[index] = max_layers + 1
        return self.dp[index]

        
#    (6, 5)
#    (5, 6)
# (3, 2) (2, 3)