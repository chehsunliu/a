class Solution:
    def merge(self, intervals: list[list[int]]) -> list[list[int]]:
        intervals = sorted(intervals)
        
        ans = []
        
        i0 = intervals[0]
        for i in range(1, len(intervals)):
            i1 = intervals[i]

            if i0[1] >= i1[0]:
                i0 = [min(i0[0], i1[0]), max(i0[1], i1[1])]
            else:
                ans.append(i0)
                i0 = i1
                
        ans.append(i0)
        return ans