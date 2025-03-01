class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        m = {}
        for i, v in enumerate(numbers):
            if v not in m:
                m[v] = set()
            m[v].add(i)

        for i, v in enumerate(numbers):
            if target - v not in m:
                continue

            js = m[target - v]

            if target == v * 2:
                if len(js) == 1:
                    continue
                js.remove(i)

            j = list(js)[0]
            return [min(i, j) + 1, max(i, j) + 1]

        raise Exception("GG")
