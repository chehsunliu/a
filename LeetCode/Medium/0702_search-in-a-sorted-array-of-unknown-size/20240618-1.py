# """
# This is ArrayReader's API interface.
# You should not implement it, or speculate about its implementation
# """
#class ArrayReader:
#    def get(self, index: int) -> int:


class Solution:
    def search(self, reader: 'ArrayReader', target: int) -> int:
        l, r = 0, 10000 - 1

        while l <= r:
            m = (l + r) // 2
            v = reader.get(m)

            if v == 2147483647:
                r = m - 1
            elif v == target:
                return m
            elif v > target:
                r = m - 1
            else:
                l = m + 1

        return -1
