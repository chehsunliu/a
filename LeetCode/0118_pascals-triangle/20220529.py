class Solution:
    def generate(self, numRows: int) -> list[list[int]]:
        rows = [[1]]
        
        for i in range(1, numRows):
            last_row = rows[i - 1]
            row = []
            
            for j in range(i + 1):
                if j == 0 or j == i:
                    row.append(1)
                else:
                    row.append(last_row[j - 1] + last_row[j])
                    
            rows.append(row)
            
        return rows