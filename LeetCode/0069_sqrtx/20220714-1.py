class Solution:
    def mySqrt(self, x: int) -> int:
        if x == 0:
            return 0
        
        l, r = 1, x
        
        while l <= r:
            m = (l + r) // 2
            m_square = m * m
            
            if m_square == x:
                return m
            elif m_square < x:
                l = m + 1
            else:
                r = m - 1
                
        return r