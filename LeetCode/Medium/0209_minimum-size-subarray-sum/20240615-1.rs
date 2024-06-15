use std::cmp::min;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;

        let mut current_sum = nums[0];
        let mut l: usize = 0;
        let mut r: usize = 0;

        while r < nums.len() {
            if current_sum >= target {
                ans = min(ans, r as i32 + 1 - l as i32);
                current_sum -= nums[l];
                l += 1;
            } else {
                r += 1;
                if r < nums.len() {
                    current_sum += nums[r];
                }
            }
        }

        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}

// 2 3 1 2 4 3
// ^            2
// ^ ^          5
// ^ ^ ^        6
// ^ ^ ^ ^      8 --> 4
//   ^ ^ ^      6
//   ^ ^ ^ ^   10 --> 4
//     ^ ^ ^    7 --> 3
//       ^ ^    6
//       ^ ^ ^  9 --> 3
//         ^ ^  7 --> 2
//           ^  3

struct Solution;
