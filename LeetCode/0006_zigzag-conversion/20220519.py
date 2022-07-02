class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1:
            return s
        
        data = ["" for _ in range(numRows)]
        
        offset = 0
        should_increase = True

        for c in s:
            data[offset] += c
            
            if should_increase:
                if offset == numRows - 1:
                    offset -= 1
                    should_increase = False
                else:
                    offset += 1
                    
            else:
                if offset == 0:
                    offset += 1
                    should_increase = True
                else:
                    offset -= 1
                    
        return "".join(data)
