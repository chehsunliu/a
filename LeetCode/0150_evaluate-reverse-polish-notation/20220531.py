class Solution:
    def evalRPN(self, tokens: list[str]) -> int:
        stack = []
        
        operations = {
            "+": lambda x, y: x + y,
            "-": lambda x, y: x - y,
            "*": lambda x, y: x * y,
            "/": lambda x, y: x // y if x * y >= 0 else -((-x) // y),
        }
        
        for token in tokens:
            if token in operations:
                f = operations[token]
                y = stack.pop()
                x = stack.pop()
                stack.append(f(x, y))
                
            else:
                stack.append(int(token))
                
        return stack[0]
