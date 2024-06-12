class Solution:
    def canVisitAllRooms(self, rooms: list[list[int]]) -> bool:
        visited = [False for _ in range(len(rooms))]
        queue = deque()
        queue.append(0)
        
        while queue:
            i = queue.popleft()
            visited[i] = True

            for j in rooms[i]:
                if not visited[j]:
                    queue.append(j)
                    
        return sum(visited) == len(rooms)