# """
# This is ArrayReader's API interface.
# You should not implement it, or speculate about its implementation
# """
#class ArrayReader:
#    def get(self, index: int) -> int:


def exist(value: int) -> bool:
    return value != (1 << 31) - 1

    
def find_boundary(reader: 'ArrayReader') -> int:
    l, r = 0, 10001

    while l + 1 < r:
        m = (l + r) // 2
        value = reader.get(m)

        if exist(value):
            l = m
        else:
            r = m

    return r


class Solution:
    def search(self, reader: 'ArrayReader', target: int) -> int:
        l, r = 0, find_boundary(reader) - 1
        
        while l <= r:
            m = (l + r) // 2
            value = reader.get(m)
            
            if value == target:
                return m
            elif value < target:
                l = m + 1
            else:
                r = m - 1
                
        return -1

                
# 0 1 2 3 4 5 6 7 8 9
# 0 1 2 3 4 5 6 7 x x