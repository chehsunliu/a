class Info:
    def __init__(self, i: int, t: int):
        self.i = i
        self.t = t


class Solution:
    def dailyTemperatures(self, temperatures: list[int]) -> list[int]:
        answer = [0 for _ in temperatures]
        stack = []
        
        for i, t in enumerate(temperatures):
            while stack:
                top = stack[-1]
                if top.t < t:
                    answer[top.i] = i - top.i
                    stack.pop()
                else:
                    break

            stack.append(Info(i, t))
                    
        return answer