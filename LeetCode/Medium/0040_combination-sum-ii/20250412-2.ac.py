class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates = sorted(candidates)
        out = []

        def f(index: int, target_left: int, tmp: list):
            if target_left == 0:
                out.append(tmp.copy())
                return

            if target_left < 0:
                return

            for i in range(index, len(candidates)):
                if i > index and candidates[i] == candidates[i - 1]:
                    continue

                tmp.append(candidates[i])
                f(i + 1, target_left - candidates[i], tmp)
                tmp.pop()

        f(0, target, [])
        return out

# 1 2 2 2 5