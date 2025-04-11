class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        ans = set()
        f(sorted(candidates), 0, target, (), ans)
        return [list(a) for a in ans]


def f(candidates: list[int], i: int, target: int, v: tuple, vs: set[tuple]):
    if target < 0:
        return

    if target == 0:
        vs.add(v)
        return

    if i >= len(candidates):
        return

    f(candidates, i + 1, target, v, vs)

    v += (candidates[i],)
    f(candidates, i + 1, target - candidates[i], v, vs)
