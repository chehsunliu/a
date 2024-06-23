use std::cmp::{max, min};

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            if nums.len() - i < ans {
                break;
            }

            let mut local_min = nums[i];
            let mut local_max = nums[i];
            for j in i..nums.len() {
                local_min = min(local_min, nums[j]);
                local_max = max(local_max, nums[j]);
                if local_max - local_min <= limit {
                    ans = max(ans, j - i + 1);
                } else {
                    break;
                }
            }
        }

        ans as i32
    }
}

struct Solution;

//  0  1  2  3  4  5
// 10  1  2  4  7  2
