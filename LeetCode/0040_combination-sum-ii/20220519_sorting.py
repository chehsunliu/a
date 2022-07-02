class Solution:
    def combinationSum2(self, candidates: list[int], target: int) -> list[list[int]]:
        candidates = sorted(candidates)

        self.answers: list[list[int]] = []
        self.f(candidates, target, [])

        return self.answers
    
    def f(self, nums: list[int], target: int, answer: list[int]):
        if target == 0:
            self.answers.append(answer)
            return
        
        elif target < 0:
            return

        for i in range(len(nums)):
            if i > 0 and nums[i - 1] == nums[i]:
                continue

            self.f(nums[i + 1:], target - nums[i], answer + [nums[i]])
            
