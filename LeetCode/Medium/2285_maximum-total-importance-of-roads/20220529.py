
class Solution:
    def maximumImportance(self, n: int, roads: list[list[int]]) -> int:
        cities = [0 for _ in range(n)]
        
        for i, j in roads:
            cities[i] += 1
            cities[j] += 1
            
        indexes = sorted(range(n), key=lambda i: cities[i])
        for i, index in enumerate(indexes, 1):
            cities[index] = i
            
        ans = 0
        for i, j in roads:
            ans += cities[i] + cities[j]
            
        return ans