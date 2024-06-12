class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        l, r = 1, num
        
        while l <= r:
            m = (l + r) // 2
            m_square = m * m
            
            if m_square == num:
                return True
            elif m_square < num:
                l = m + 1
            else:
                # m_square > num
                r = m - 1
            
        return False