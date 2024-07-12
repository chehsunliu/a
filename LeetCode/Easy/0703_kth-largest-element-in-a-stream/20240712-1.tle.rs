use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k,
            heap: BinaryHeap::from(nums),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(val);
        let mut tmp = vec![];
        for _ in 1..self.k {
            tmp.push(self.heap.pop().unwrap());
        }
        let v = self.heap.pop().unwrap();
        for t in tmp {
            self.heap.push(t);
        }
        self.heap.push(v);
        v
    }
}

// 2 3 4 5 5 8 8 8 10

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {}
}
