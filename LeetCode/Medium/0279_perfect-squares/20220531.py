class Solution:
    def numSquares(self, n: int) -> int:
        numbers = [i * i for i in range(100, 0, -1)]
        ns = [None for _ in range(n + 1)]
        ns[n] = 0
        queue = deque()
        queue.append(n)
        
        while queue:
            current_n = queue.popleft()
            steps = ns[current_n]
            
            if current_n == 0:
                return steps
            
            for number in numbers:
                next_n = current_n - number
                if next_n < 0 or ns[next_n] is not None:
                    continue

                ns[next_n] = steps + 1
                queue.append(next_n)
                
        raise Exception("GG")