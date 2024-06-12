class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        l, r = 0, len(numbers) - 1
        
        while True:
            s = numbers[l] + numbers[r] - target
            
            if s == 0:
                return [l + 1, r + 1]
            elif s > 0:
                r -= 1
            else:
                l += 1