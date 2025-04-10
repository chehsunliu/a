class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        ans = []
        f(candidates, 0, target, [], ans)
        return ans


def f(candidates: list[int], i: int, target: int, v: list[int], vs: list[list[int]]):
    if target < 0:
        return

    if target == 0:
        vs.append(v)
        return

    if i >= len(candidates):
        return

    f(candidates, i + 1, target, v, vs)

    v = v.copy()
    v.append(candidates[i])
    f(candidates, i, target - candidates[i], v, vs)
