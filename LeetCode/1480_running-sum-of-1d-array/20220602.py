class Solution:
    def runningSum(self, nums: list[int]) -> list[int]:
        results = []
        
        result = 0
        for num in nums:
            result += num
            results.append(result)
            
        return results