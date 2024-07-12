use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                heap.push(Reverse(matrix[i][j]));
            }
        }

        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap().0
    }
}

//  1  5  9
// 10 11 13
// 12 13 15

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
