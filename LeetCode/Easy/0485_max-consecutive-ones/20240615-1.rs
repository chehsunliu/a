use std::cmp::max;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                ans = max(ans, i - l);
                l = i + 1;
            }
        }
        ans = max(ans, nums.len() - l);

        ans as i32
    }
}

struct Solution;
