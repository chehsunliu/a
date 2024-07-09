use std::cmp::max;
use std::collections::VecDeque;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut queue: VecDeque<usize> = VecDeque::new();

        for i in 0..height.len() {
            if let Some(&i0) = queue.front() {
                if height[i0] <= height[i] {
                    while let Some(j) = queue.pop_back() {
                        ans += height[i0] - height[j];
                    }
                }
            }
            queue.push_back(i);
        }

        let mut max_height = 0;
        while let Some(i) = queue.pop_back() {
            max_height = max(max_height, height[i]);
            ans += max_height - height[i];
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
