# 1 2 3 4 5
# 1 2 3 5 4
# 1 2 4 3 5
# 1 2 4 5 3
# 1 2 5 3 4
# 1 2 5 4 3
#     ^
# 1 3 2 4 5
#         ^
# 1 3 2 5 4
#       ^
# 1 3 4 2 5

class Solution:
    def permute(self, nums: list[int]) -> list[list[int]]:
        count = 1
        for i in range(1, len(nums) + 1):
            count *= i
        
        answer = [nums]
        for i in range(count - 1):
            nums = self.next_permutation(nums)
            answer.append(nums)
        return answer
    
    def next_permutation(self, nums: list[int]) -> list[int]:
        pivot = len(nums) - 1
        while nums[pivot - 1] >= nums[pivot] and pivot > 0:
            pivot -= 1
            
        if pivot == 0:
            return list(reversed(nums))
        
        index = None
        for i in range(len(nums) - 1, -1, -1):
            if nums[i] > nums[pivot - 1]:
                index = i
                break

        nums = nums[:]
        nums[pivot - 1], nums[index] = nums[index], nums[pivot - 1]
        nums[pivot:len(nums)] = sorted(nums[pivot:len(nums)])
        return nums
        