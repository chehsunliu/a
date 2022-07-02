class Solution:
    def openLock(self, deadends: list[str], target: str) -> int:
        deadends = set(deadends)
        if "0000" in deadends:
            return -1

        queue = deque()
        queue.append("0000")
        records = {}
        records["0000"] = 0
        
        while queue:
            current_lock = queue.popleft()
            value = records[current_lock] + 1
            
            locks = []
            for digit in range(4):
                c_plus = str((int(current_lock[digit]) + 1) % 10)
                c_minus = str((int(current_lock[digit]) - 1 + 10) % 10)
                locks.append(current_lock[:digit] + c_plus + current_lock[digit + 1:])
                locks.append(current_lock[:digit] + c_minus + current_lock[digit + 1:])
                
            for lock in locks:
                if lock in deadends:
                    continue
                    
                if lock in records:
                    continue
                    
                records[lock] = value
                queue.append(lock)
                
        return records[target] if target in records else -1