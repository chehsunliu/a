class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        q = deque()

        ans = [0 for _ in range(len(temperatures))]

        for i in range(len(temperatures)):
            while len(q) > 0 and temperatures[q[0]] < temperatures[i]:
                ans[q[0]] = i - q[0]
                q.popleft()
            q.appendleft(i)

        return ans
