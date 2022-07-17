class Solution:
    @cache
    def myPow(self, x: float, n: int) -> float:
        if n < 0:
            return 1 / self.myPow(x, -n)
        
        if n == 0:
            return 1
        elif n == 1:
            return x
        
        p = n // 2
        r = n % 2
        return self.myPow(x, p) * self.myPow(x, p + r)