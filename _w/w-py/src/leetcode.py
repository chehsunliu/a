class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        l, r = 0, len(numbers) - 1

        # 1 2 3 4 5
        while l < r:
            if target == numbers[l] + numbers[r]:
                return [l + 1, r + 1]

            if target > numbers[l] + numbers[r]:
                l += 1
            else:
                r -= 1

        raise Exception("GG")
