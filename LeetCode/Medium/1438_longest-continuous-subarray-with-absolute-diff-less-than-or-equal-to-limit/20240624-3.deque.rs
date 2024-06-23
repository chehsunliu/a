use std::cmp::{max, min};
use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;

        let mut max_deque: VecDeque<i32> = VecDeque::new();
        let mut min_deque: VecDeque<i32> = VecDeque::new();

        for right in 0..nums.len() {
            while max_deque.len() > 0 && *max_deque.back().unwrap() < nums[right] {
                max_deque.pop_back();
            }
            max_deque.push_back(nums[right]);

            while min_deque.len() > 0 && *min_deque.back().unwrap() > nums[right] {
                min_deque.pop_back();
            }
            min_deque.push_back(nums[right]);

            while max_deque.front().unwrap() - min_deque.front().unwrap() > limit {
                if *max_deque.front().unwrap() == nums[left] {
                    max_deque.pop_front();
                }

                if *min_deque.front().unwrap() == nums[left] {
                    min_deque.pop_front();
                }

                left += 1;
            }

            ans = max(ans, right - left + 1);
        }

        ans as i32
    }
}

struct Solution;
