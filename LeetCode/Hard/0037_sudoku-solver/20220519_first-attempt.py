class InvalidAnswer(Exception):
    pass


class Sudoku:
    def __init__(self, records: list[list[set[int]]]) -> None:
        self.records = records

    def solve(self) -> list[list[set[int]]]:
        prev_count = self._count()
        while True:
            self._update_by_rows()
            self._update_by_cols()
            self._update_by_blocks()
            
            self._update_by_row_singletons()
            self._update_by_col_singletons()
            self._update_by_block_singletons()
            
            count = self._count()
            if prev_count == count:
                break
            else:
                prev_count = count
        
        if prev_count == 81:
            self._raise_if_not_valid()
            self._print()
            return self.records

        return self._assume_and_solve()
    
    def _assume_and_solve(self) -> list[list[set[int]]]:
        queue = []
        for r in range(9):
            for c in range(9):
                values = self.records[r][c]
                if len(values) > 1:
                    queue.append((r, c, values))
                    
        for r0, c0, values in sorted(queue, key=lambda x: len(x[2])):
            records_copy = [[self.records[r][c].copy() for c in range(9)] for r in range(9)]
            for value in values:
                records_copy[r0][c0] = {value}
                try:
                    return Sudoku(records_copy).solve()
                except InvalidAnswer:
                    pass
    
        raise InvalidAnswer("QQ")
    
    def _raise_if_not_valid(self) -> None:
        for r in range(9):
            values = set()

            for c in range(9):
                value = self.records[r][c]
                if len(value) != 1:
                    raise InvalidAnswer("QQ")
                values.add(list(value)[0])

            if sum(values) != 45:
                raise InvalidAnswer("QQ")

        for c in range(9):
            values = set()

            for r in range(9):
                value = self.records[r][c]
                values.add(list(value)[0])

            if sum(values) != 45:
                raise InvalidAnswer("QQ")
                
        for r_base in range(0, 9, 3):
            for c_base in range(0, 9, 3):
                values = set()

                for r_offset in range(3):
                    for c_offset in range(3):
                        r = r_base + r_offset
                        c = c_base + c_offset
                        value = self.records[r][c]
                        values.add(list(value)[0])

                if sum(values) != 45:
                    raise InvalidAnswer("QQ")
    
    def _count(self) -> int:
        count = 0
        for r in range(9):
            for c in range(9):
                count += len(self.records[r][c])
        return count
        
    def _update_by_rows(self) -> None:
        for r in range(9):
            existing_values = set()
            for c in range(9):
                values = self.records[r][c]
                if len(values) == 1:
                    existing_values.add(list(values)[0])
                    
            for c in range(9):
                values = self.records[r][c]
                if len(values) > 1:
                    self.records[r][c].difference_update(existing_values)
                    
    def _update_by_cols(self) -> None:
        for c in range(9):
            existing_values = set()
            for r in range(9):
                values = self.records[r][c]
                if len(values) == 1:
                    existing_values.add(list(values)[0])
                    
            for r in range(9):
                values = self.records[r][c]
                if len(values) > 1:
                    self.records[r][c].difference_update(existing_values)
    
    def _update_by_blocks(self) -> None:
        for r_base in range(0, 9, 3):
            for c_base in range(0, 9, 3):
                existing_values = set()
                for r_offset in range(3):
                    for c_offset in range(3):
                        r = r_base + r_offset
                        c = c_base + c_offset
                        values = self.records[r][c]
                        if len(values) == 1:
                            existing_values.add(list(values)[0])
                
                for r_offset in range(3):
                    for c_offset in range(3):
                        r = r_base + r_offset
                        c = c_base + c_offset
                        values = self.records[r][c]
                        if len(values) > 1:
                            self.records[r][c].difference_update(existing_values)

    def _update_by_row_singletons(self) -> None:
        for r in range(9):
            for value in range(1, 10):
                count = 0
                target_c = -1
                for c in range(9):
                    if value in self.records[r][c]:
                        count += 1
                        target_c = c
                if count == 1:
                    self.records[r][target_c] = {value}

    def _update_by_col_singletons(self) -> None:
        for c in range(9):
            for value in range(1, 10):
                count = 0
                target_r = -1
                for r in range(9):
                    if value in self.records[r][c]:
                        count += 1
                        target_r = r
                if count == 1:
                    self.records[target_r][c] = {value}
                    
    def _update_by_block_singletons(self) -> None:
        for r_base in range(0, 9, 3):
            for c_base in range(0, 9, 3):
                for value in range(1, 10):
                    count = 0
                    target_r = -1
                    target_c = -1
                    for r_offset in range(3):
                        for c_offset in range(3):
                            r = r_base + r_offset
                            c = c_base + c_offset
                            if value in self.records[r][c]:
                                count += 1
                                target_r = r
                                target_c = c
                    if count == 1:
                        self.records[target_r][target_c] = {value}
    
    def _count(self) -> int:
        count = 0
        for r in range(9):
            for c in range(9):
                count += len(self.records[r][c])
        return count
    
    def _print(self) -> None:
        print("----")
        for row in self.records:
            for col in row:
                value = ",".join([str(v) for v in col])
                print("%12s " % value, end="")
            print()


class Solution:
    def solveSudoku(self, board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        
        tmp = set([i + 1 for i in range(9)])
        records = [[tmp.copy() for _ in range(9)] for _ in range(9)]
        
        for r in range(9):
            for c in range(9):
                value = board[r][c]
                if value != ".":
                    records[r][c] = {int(value)}
        
        records = Sudoku(records).solve()

        for r in range(9):
            for c in range(9):
                board[r][c] = str(list(records[r][c])[0])
