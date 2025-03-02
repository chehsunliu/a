class Solution:
    def findMin(self, nums: List[int]) -> int:
        if len(nums) == 1 or nums[-1] > nums[0]:
            return nums[0]

        l, r = 0, len(nums) - 1

        while l < r:
            m = (l + r) // 2

            if nums[m] < nums[-1]:
                r = m
            else:
                l = m + 1

        return nums[l]



# 0 1 2 3 4
# 4 5 1 2 3
# l   ^   r
# l ^ r

# 0 1 2 3 4 5 6
# 3 4 5 1 2
# l   ^   r
#       l^r
#     r l

# 6 7 1 2 3 4 5
# l     ^     r
# l ^ r
#   r l
# 3 4 5 6 7 1 2
#       ^
#           ^
