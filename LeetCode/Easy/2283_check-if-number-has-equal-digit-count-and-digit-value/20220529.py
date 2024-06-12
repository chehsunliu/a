class Solution:
    def digitCount(self, num: str) -> bool:
        count = [0 for _ in range(10)]
        for c in num:
            count[int(c)] += 1
            
        return all([count[i] == int(num[i]) for i in range(len(num))])