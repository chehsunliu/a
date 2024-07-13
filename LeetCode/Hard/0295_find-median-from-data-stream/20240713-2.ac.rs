use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    greater_nums: BinaryHeap<Reverse<i32>>,
    smaller_nums: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            greater_nums: BinaryHeap::new(),
            smaller_nums: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.greater_nums.is_empty() || num >= self.greater_nums.peek().unwrap().0 {
            self.greater_nums.push(Reverse(num));
        } else if self.smaller_nums.is_empty() || num <= *self.smaller_nums.peek().unwrap() {
            self.smaller_nums.push(num);
        } else {
            self.greater_nums.push(Reverse(num));
        }

        if self.greater_nums.len() > self.smaller_nums.len() + 1 {
            self.smaller_nums.push(self.greater_nums.pop().unwrap().0);
        } else if self.smaller_nums.len() > self.greater_nums.len() + 1 {
            self.greater_nums
                .push(Reverse(self.smaller_nums.pop().unwrap()));
        }
    }

    fn find_median(&mut self) -> f64 {
        if self.greater_nums.len() == self.smaller_nums.len() {
            ((self.greater_nums.peek().unwrap().0 as f64)
                + (*self.smaller_nums.peek().unwrap() as f64))
                / 2.0
        } else if self.greater_nums.len() > self.smaller_nums.len() {
            self.greater_nums.peek().unwrap().0 as f64
        } else {
            *self.smaller_nums.peek().unwrap() as f64
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
