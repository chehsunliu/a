class Solution:
    def combinationSum2(self, candidates: list[int], target: int) -> list[list[int]]:
        self.dp: dict[int, set[tuple[int]]] = {}
        
        return [list(value) for value in self.f(target, candidates)]
    
    def f(self, target: int, candidates: list[int]) -> set[tuple[int]]:
        if target in self.dp:
            answers = set()
            for answer in self.dp[target]:
                candidates_copy = candidates[:]
                is_valid = True
                for value in answer:
                    try:
                        candidates_copy.remove(value)
                    except ValueError:
                        is_valid = False
                        break
                if is_valid:
                    answers.add(answer)
            return answers

        answers: set[tuple[int]] = set()

        for candidate in candidates:
            target_next = target - candidate
            if target_next == 0:
                answers.add((candidate,))
            
            elif target_next > 0:
                candidates_next = candidates[:]
                candidates_next.remove(candidate)

                for answer_next in self.f(target_next, candidates_next):
                    answer = tuple(sorted((*answer_next, candidate)))
                    answers.add(answer)
            
            else:
                pass
            
        self.dp[target] = answers
        return answers