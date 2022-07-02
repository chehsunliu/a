class Solution:
    def wallsAndGates(self, rooms: list[list[int]]) -> None:
        m = len(rooms)
        n = len(rooms[0])
        
        gates = []
        directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]

        for i in range(m):
            for j in range(n):
                if rooms[i][j] == 0:
                    gates.append((i, j, 0))
                
        for gate in gates:
            queue = deque()
            queue.append(gate)

            while queue:
                i, j, distance = queue.popleft()
                distance += 1
                
                for offset_i, offset_j in directions:
                    next_i, next_j = i + offset_i, j + offset_j
                    if (0 <= next_i < m) and (0 <= next_j < n) and rooms[next_i][next_j] > distance:
                        rooms[next_i][next_j] = distance
                        queue.append((next_i, next_j, distance))