use std::collections::BinaryHeap;

struct MedianFinder {
    heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.heap.push(num);
    }

    fn find_median(&mut self) -> f64 {
        let count = self.heap.len();

        let mut tmp = vec![];
        for i in 0..(count / 2) {
            tmp.push(self.heap.pop().unwrap());
        }
        let median = if count % 2 == 1 {
            *self.heap.peek().unwrap() as f64
        } else {
            ((*self.heap.peek().unwrap()) as f64 + (*tmp.last().unwrap()) as f64) / 2.0
        };

        for v in tmp {
            self.heap.push(v);
        }
        return median;
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
