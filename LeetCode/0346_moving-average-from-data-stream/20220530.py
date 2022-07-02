class MovingAverage:

    def __init__(self, size: int):
        self.data = [None for _ in range(size)]
        self.head = 0
        self.tail = 0
        self.current_sum = 0

    def next(self, val: int) -> float:
        if self.data[self.tail] is None:
            self.current_sum += val

            self.data[self.tail] = val
            self.tail = (self.tail + 1) % len(self.data)
            
            size = (self.tail - self.head + len(self.data)) % len(self.data)
            size = size if size != 0 else len(self.data)
            return self.current_sum / size

        else:
            self.current_sum += val - self.data[self.head]

            self.data[self.head] = val
            self.head = (self.head + 1) % len(self.data)
            self.tail = self.head

            return self.current_sum / len(self.data)


# Your MovingAverage object will be instantiated and called as such:
# obj = MovingAverage(size)
# param_1 = obj.next(val)