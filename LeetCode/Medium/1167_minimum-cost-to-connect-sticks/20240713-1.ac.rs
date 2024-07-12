use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn connect_sticks(mut sticks: Vec<i32>) -> i32 {
        if sticks.len() == 1 {
            return 0;
        }

        let mut queue = BinaryHeap::new();
        for v in sticks {
            queue.push(Reverse(v));
        }

        let mut ans = 0;
        while queue.len() >= 2 {
            let v = queue.pop().unwrap().0 + queue.pop().unwrap().0;
            queue.push(Reverse(v));
            ans += v;
        }

        ans
    }
}

// 32 33 43 45 49
// 65 43 45 49 -> 65

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
