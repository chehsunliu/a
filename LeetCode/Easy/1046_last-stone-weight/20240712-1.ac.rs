use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let (s1, s2) = (heap.pop().unwrap(), heap.pop().unwrap());
            if s1 != s2 {
                heap.push((s2 - s1).abs());
            }
        }

        heap.pop().unwrap_or(0)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
