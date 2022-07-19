class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        i1, i2 = 0, 0
        m0, m1 = 0, 0

        while i1 + i2 <= (len(nums1) + len(nums2)) // 2:
            m0 = m1

            if i1 < len(nums1) and i2 < len(nums2):
                if nums1[i1] < nums2[i2]:
                    m1 = nums1[i1]
                    i1 += 1
                else:
                    m1 = nums2[i2]
                    i2 += 1
            elif i1 < len(nums1):
                m1 = nums1[i1]
                i1 += 1
            else:
                m1 = nums2[i2]
                i2 += 1
                
        if (len(nums1) + len(nums2)) % 2 == 0:
            return (m0 + m1) / 2
        else:
            return m1
        