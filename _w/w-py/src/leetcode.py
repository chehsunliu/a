import hashlib


class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        lookup = {}

        for s in strs:
            v = hashlib.md5("".join(sorted(s)).encode("utf-8")).hexdigest()
            if v not in lookup:
                lookup[v] = []
            lookup[v].append(s)

        ans = []
        for vs in lookup.values():
            ans.append(vs)
        return ans
