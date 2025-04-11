class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        self.out = set()
        self.candidates = sorted(candidates)

        self.f(0, target, ())
        return [list(vs) for vs in self.out]

    def f(self, i: int, target: int, vs: tuple):
        if target == 0:
            self.out.add(vs)
            return

        if target < 0:
            return

        if i >= len(self.candidates):
            return

        self.f(i + 1, target, vs)

        vs += (self.candidates[i],)
        self.f(i + 1, target - self.candidates[i], vs)

# 1 2 2 2 5