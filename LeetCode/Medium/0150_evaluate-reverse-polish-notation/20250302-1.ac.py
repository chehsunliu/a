class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stk = []

        for s in tokens:
            if s == "+":
                expr2 = stk.pop()
                expr1 = stk.pop()
                stk.append(expr1 + expr2)
            elif s == "-":
                expr2 = stk.pop()
                expr1 = stk.pop()
                stk.append(expr1 - expr2)
            elif s == "*":
                expr2 = stk.pop()
                expr1 = stk.pop()
                stk.append(expr1 * expr2)
            elif s == "/":
                expr2 = stk.pop()
                expr1 = stk.pop()
                v = expr1 / expr2
                if v > 0:
                    stk.append(math.floor(v))
                else:
                    stk.append(math.ceil(v))
            else:
                stk.append(int(s))

        return stk[-1]