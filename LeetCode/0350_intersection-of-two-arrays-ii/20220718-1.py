def search_l(arr: list[int], target: int) -> int:
    l, r = 0, len(arr)
    
    while l < r:
        m = (l + r) // 2
        
        if arr[m] < target:
            l = m + 1
        else:
            r = m
            
    return l


class Solution:
    def intersect(self, nums1: list[int], nums2: list[int]) -> list[int]:
        nums1, nums2 = sorted(nums1), sorted(nums2)
        
        i1 = 0
        ans =[]
        
        while i1 < len(nums1):
            x = nums1[i1]
            j1 = search_l(nums1, x + 1)
            count1 = j1 - i1
            
            i2 = search_l(nums2, x)
            if i2 < len(nums2) and nums2[i2] == x:
                j2 = search_l(nums2, x + 1)
                count2 = j2 - i2
                ans += [x] * min(count1, count2)

            i1 = j1

        return ans