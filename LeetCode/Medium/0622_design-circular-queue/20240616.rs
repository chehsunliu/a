struct MyCircularQueue {
    storage: Vec<i32>,
    l: usize,
    r: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            storage: vec![0; k as usize + 1],
            l: 0,
            r: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.storage[self.r] = value;
        self.r = (self.r + 1) % self.storage.len();
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.l = (self.l + 1) % self.storage.len();
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.storage[self.l]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.storage[(self.r + self.storage.len() - 1) % self.storage.len()]
        }
    }

    fn is_empty(&self) -> bool {
        self.l == self.r
    }

    fn is_full(&self) -> bool {
        (self.l + self.storage.len() - self.r) % self.storage.len() == 1
    }
}
