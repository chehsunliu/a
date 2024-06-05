def bs(arr: list[int], target: int, l: int, r: int) -> tuple[int, int]:
    while l < r:
        m = (l + r) // 2
        
        if arr[m] == target:
            return m, m
        elif arr[m] < target:
            l = m + 1
        else:
            r = m - 1
            
    return l, r


class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        l, r = 0, len(numbers) - 1
        
        while l < r:
            s = numbers[l] + numbers[r]
            if s == target:
                return [l + 1, r + 1]

            if s < target:
                result = bs(numbers, target - numbers[r], l + 1, r - 1)
                l = result[0]
            else:
                result = bs(numbers, target - numbers[l], l + 1, r - 1)
                r = result[1]


#  0  1  2  3  4  5  6  7  8  9 10
#  1  3  3  9 14 23 27 31 62 63 90 | 37
#
# i0: 1+90=91 > 37, find 37-1=36 among 1..9, found 31 at 7
# i1: 1+31=32 < 37, find 37-31=6 among 1..6, found 9 at 3
# i2: 9+31=40 > 37, find 37-9=28 among 4..6, found 27 at 6
# i3: 9+27=36 < 37, find 37-27=10 among 4..5, found 14 at 4
# i4: 14+27=41 > 37, find 37-14=23 among 5..5, found 23 at 5

