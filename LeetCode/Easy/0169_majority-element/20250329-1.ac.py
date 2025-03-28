class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        m = {}
        ans = nums[0]

        for v in nums:
            if v not in m:
                m[v] = 0
            m[v] += 1
            ans = ans if m[ans] >= m[v] else v

        return ans