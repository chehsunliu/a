def search(arr: list[int], target: int) -> int:
    l, r = 0, len(arr) - 1
    
    while l <= r:
        m = (l + r) // 2
        
        if arr[m] == target:
            return m
        elif arr[m] < target:
            l = m + 1
        else:
            r = m - 1
            
    return l


class Solution:
    def findClosestElements(self, arr: list[int], k: int, x: int) -> list[int]:
        c = search(arr, x)
        
        if c >= len(arr):
            c = len(arr) - 1
        elif c == 0:
            pass
        else:
            if x - arr[c - 1] <= arr[c] - x:
                c -= 1
        
        l, r = c, c

        while r - l + 1 < k:
            if l - 1 >= 0 and r + 1 < len(arr):
                if x - arr[l - 1] <= arr[r + 1] - x:
                    l -= 1
                else:
                    r += 1
            elif l - 1 >= 0:
                l -= 1
            else:
                r += 1
                
        return arr[l : r + 1]
                    
        
# x=36, k=4
#  0  1  2 11 15 19 20 34 36 67 68 71 87 91
# 36 35 34 25 21 17 16  2  0 31 32 35 51 55
#                       ^  ^  ^  ^
#           ^
#                 ^  ^  ^  ^