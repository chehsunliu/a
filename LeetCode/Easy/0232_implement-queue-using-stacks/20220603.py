class MyQueue:

    def __init__(self):
        self.l = []
        self.r = []

    def push(self, x: int) -> None:
        self.l.append(x)

    def pop(self) -> int:
        if not self.r:
            while self.l:
                self.r.append(self.l.pop())
        return self.r.pop()

    def peek(self) -> int:
        if not self.r:
            while self.l:
                self.r.append(self.l.pop())
        return self.r[-1]

    def empty(self) -> bool:
        return len(self.l) + len(self.r) == 0
        


# Your MyQueue object will be instantiated and called as such:
# obj = MyQueue()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.peek()
# param_4 = obj.empty()