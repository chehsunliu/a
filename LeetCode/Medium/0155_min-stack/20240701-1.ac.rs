use std::cmp::min;
use std::collections::VecDeque;

struct MinStack {
    numbers: VecDeque<i32>,
    mins: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            numbers: VecDeque::new(),
            mins: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.numbers.push_back(val);
        self.mins
            .push_back(min(self.mins.back().map_or(i32::MAX, |v| *v), val));
    }

    fn pop(&mut self) {
        self.numbers.pop_back();
        self.mins.pop_back();
    }

    fn top(&self) -> i32 {
        *self.numbers.back().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.back().unwrap()
    }
}

// 12 14 8
// 12 12 8
