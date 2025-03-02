class Solution:
    def search(self, nums: List[int], target: int) -> int:
        pivot = self.find_pivot(nums)

        index1 = self.search_partial(nums, (0, pivot - 1), target)
        index2 = self.search_partial(nums, (pivot, len(nums) - 1), target)

        if index1 >= 0:
            return index1
        if index2 >= 0:
            return index2
        return -1

    def search_partial(self, nums: list[int], indexes: tuple[int, int], target: int):
        l, r = indexes

        while l <= r:
            m = (l + r) // 2
            if nums[m] == target:
                return m
            elif nums[m] < target:
                l = m + 1
            else:
                r = m - 1

        return -1

    def find_pivot(self, nums: list[int]) -> int:
        if len(nums) == 1 or nums[0] < nums[-1]:
            return 0

        l, r = 0, len(nums) - 1
        while l < r:
            m = (l + r) // 2

            if nums[m] < nums[-1]:
                r = m
            else:
                l = m + 1

        return r