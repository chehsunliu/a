class MyCircularQueue:

    def __init__(self, k: int):
        self.data = [None for _ in range(k)]
        self.head = 0
        self.tail = 0

    def enQueue(self, value: int) -> bool:
        if self.data[self.tail] is not None:
            return False

        self.data[self.tail] = value
        self.tail = (self.tail + 1) % len(self.data)
        return True

    def deQueue(self) -> bool:
        if self.data[self.head] is None:
            return False
        
        self.data[self.head] = None
        self.head = (self.head + 1) % len(self.data)
        return True

    def Front(self) -> int:
        return self.data[self.head] if self.data[self.head] is not None else -1

    def Rear(self) -> int:
        return self.data[self.tail - 1] if self.data[self.tail - 1] is not None else -1
        

    def isEmpty(self) -> bool:
        return self.data[self.head] is None
        

    def isFull(self) -> bool:
        return self.data[self.tail] is not None
        


# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue(k)
# param_1 = obj.enQueue(value)
# param_2 = obj.deQueue()
# param_3 = obj.Front()
# param_4 = obj.Rear()
# param_5 = obj.isEmpty()
# param_6 = obj.isFull()