class Solution:
    def addBinary(self, a: str, b: str) -> str:
        a = "0" * max(0, len(b) - len(a)) + a
        b = "0" * max(0, len(a) - len(b)) + b
            
        answer = []
        carry = 0

        for i in range(len(a) - 1, -1, -1):
            value = carry + int(a[i]) + int(b[i]) if i < len(b) else carry + int(a[i])
            answer.append(value % 2)
            carry = value // 2
            
        if carry:
            answer.append(carry)
            
        return "".join([str(v) for v in reversed(answer)])