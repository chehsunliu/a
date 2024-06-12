def print_records(records: list[list[set[int]]]) -> None:
    for r in range(9):
        for c in range(9):
            print(records[r][c] if records[r][c] else "-", " ", end="")
        print()
    print("****")


class Node:
    def __init__(self, r: int, c: int, availables: Optional[set[int]] = None) -> None:
        self.r = r
        self.c = c
        self.availables = availables
    
    
class Solution:
    def solveSudoku(self, board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        
        records = self._create_records(board)
        
        queue = []
        for r in range(9):
            for c in range(9):
                value = records[r][c]
                if not value:
                    queue.append(Node(r, c))

        index = 0
        while index < len(queue):
            node = queue[index]

            if node.availables is None:
                availables = self._get_availables(records, node.r, node.c)
                if not availables:
                    index -= 1
                    continue
                    
                node.availables = availables

            elif len(node.availables) == 0:
                node.availables = None
                records[node.r][node.c] = None
                index -= 1
                continue
            
            records[node.r][node.c] = node.availables.pop()
            index += 1

        for r in range(9):
            for c in range(9):
                board[r][c] = str(records[r][c])
        
    def _create_records(self, board: list[list[str]]) -> list[list[Optional[int]]]:
        return [[(int(board[r][c]) if board[r][c] != "." else None) for c in range(9)] for r in range(9)]

    def _get_availables(self, records: list[list[Optional[int]]], r: int, c: int) -> set[int]:
        result = set([i for i in range(1, 10)])
        
        row_elements = set([records[r][i] for i in range(9) if records[r][i]])
        col_elements = set([records[i][c] for i in range(9) if records[i][c]])
        
        block_elements = set()
        r_base = r // 3
        c_base = c // 3
        for r_offset in range(3):
            for c_offset in range(3):
                value = records[r_base * 3 + r_offset][c_base * 3 + c_offset]
                if value:
                    block_elements.add(value)
                    
        result.difference_update(row_elements)
        result.difference_update(col_elements)
        result.difference_update(block_elements)
        
        return result
