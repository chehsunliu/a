class Solution:
    def getRow(self, rowIndex: int) -> list[int]:
        row0 = [0 for _ in range(rowIndex + 1)]
        row1 = row0[:]
        
        row0[0] = 1
        
        for i in range(1, rowIndex + 1):
            row1[1:i + 1] = row0[:i]
            row1[0] = 0
            
            for j in range(i + 1):
                row0[j] += row1[j]
                
        return row0
