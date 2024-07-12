use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for v in nums {
            counts.insert(v, counts.get(&v).unwrap_or(&0) + 1);
        }

        let mut heap = BinaryHeap::new();
        for (v, count) in counts {
            heap.push((count, v));
        }

        let mut ans = vec![];
        for _ in 1..=k {
            ans.push(heap.pop().unwrap().1);
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {}
}
