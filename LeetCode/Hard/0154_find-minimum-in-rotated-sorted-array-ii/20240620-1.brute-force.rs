use std::cmp::min;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        for n in nums {
            ans = min(ans, n);
        }
        ans
    }
}

struct Solution;
