class Solution:
    def combinationSum(self, candidates: list[int], target: int) -> list[list[int]]:
        self.candidates = candidates
        self.dp: dict[int: set[tuple[int]]] = {}
        
        return [list(value) for value in self.f(target)]
    
    def f(self, target: int) -> set[tuple[int]]:
        if target in self.dp:
            return self.dp[target]

        answers: set[tuple[int]] = set()

        for candidate in self.candidates:
            target_next = target - candidate
            if target_next == 0:
                answers.add((candidate,))
            
            elif target_next > 0:
                for answer_next in self.f(target_next):
                    answer = tuple(sorted((*answer_next, candidate)))
                    answers.add(answer)
            
            else:
                pass
            
        self.dp[target] = answers
        return answers