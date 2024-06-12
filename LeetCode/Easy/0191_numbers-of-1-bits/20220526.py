class Solution:
    def hammingWeight(self, n: int) -> int:
        value = (pow(2, 32) - 1) & n
        
        answer = 0
        while value:
            answer += value & 1
            value = value >> 1
        
        return answer