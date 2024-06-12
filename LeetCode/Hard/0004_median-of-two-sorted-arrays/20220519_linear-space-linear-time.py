class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        nums = []
        i1 = i2 = 0
        
        while len(nums) < len(nums1) + len(nums2):
            if i1 < len(nums1) and i2 < len(nums2):
                if nums1[i1] <= nums2[i2]:
                    nums.append(nums1[i1])
                    i1 += 1
                else:
                    nums.append(nums2[i2])
                    i2 += 1
                    
            elif i1 < len(nums1):
                nums.append(nums1[i1])
                i1 += 1
                
            else:
                nums.append(nums2[i2])
                i2 += 1
                
        if len(nums) % 2 == 0:
            return (nums[len(nums) // 2] + nums[len(nums) // 2 - 1]) / 2
        
        return nums[len(nums) // 2]
