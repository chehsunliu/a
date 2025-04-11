class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        out = []

        def f(m: int, tmp: list):
            if len(tmp) == k:
                out.append(tmp.copy())
                return

            for i in range(m, n + 1):
                tmp.append(i)
                f(i + 1, tmp)
                tmp.pop()

        f(1, [])
        return out