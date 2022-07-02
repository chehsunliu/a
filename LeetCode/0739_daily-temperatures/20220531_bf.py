class Solution:
    def dailyTemperatures(self, temperatures: list[int]) -> list[int]:
        answer = [0 for _ in temperatures]
        
        for i in range(len(temperatures)):
            for j in range(i + 1, len(temperatures)):
                if temperatures[j] > temperatures[i]:
                    answer[i] = j - i
                    break
                    
        return answer