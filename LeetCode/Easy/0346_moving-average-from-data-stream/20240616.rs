struct MovingAverage {
    storage: Vec<i32>,
    window_sum: i32,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            storage: vec![0; size as usize],
            window_sum: 0,
            count: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.count < self.storage.len() {
            let i = self.count;
            self.window_sum += val;
            self.storage[i] = val;
            self.count += 1;
            self.window_sum as f64 / self.count as f64
        } else {
            let i = self.count % self.storage.len();
            self.window_sum += val - self.storage[i];
            self.storage[i] = val;
            self.count += 1;
            self.window_sum as f64 / self.storage.len() as f64
        }
    }
}
