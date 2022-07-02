class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        i1 = i2 = 0
        middles = (0, 0)

        n = len(nums1) + len(nums2)
        
        while i1 + i2 <= n // 2:
            if i1 < len(nums1) and i2 < len(nums2):
                if nums1[i1] <= nums2[i2]:
                    middles = (middles[1], nums1[i1])
                    i1 += 1
                else:
                    middles = (middles[1], nums2[i2])
                    i2 += 1
                    
            elif i1 < len(nums1):
                middles = (middles[1], nums1[i1])
                i1 += 1
                
            else:
                middles = (middles[1], nums2[i2])
                i2 += 1
                
        if n % 2 == 0:
            return (middles[0] + middles[1]) / 2
        
        return middles[1]
