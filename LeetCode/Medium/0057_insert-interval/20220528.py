def search_left(intervals: list[list[int]], target: int, key) -> int:
    l, r = 0, len(intervals) - 1
    
    while l <= r:
        m = (l + r) // 2
        
        if key(intervals[m]) < target:
            l = m + 1
        
        else:
            r = m - 1
            
    return l


class Solution:
    def insert(self, intervals: list[list[int]], newInterval: list[int]) -> list[list[int]]:
        l = search_left(intervals, newInterval[0], key=lambda x: x[0])
        intervals.insert(l, newInterval)
        
        ans = []
        
        prev_interval = intervals[0]
        for i in range(1, len(intervals)):
            interval = intervals[i]
            
            if prev_interval[1] >= interval[0]:
                prev_interval = [min(prev_interval[0], interval[0]), max(prev_interval[1], interval[1])]
            else:
                ans.append(prev_interval)
                prev_interval = interval
                
        ans.append(prev_interval)
        return ans