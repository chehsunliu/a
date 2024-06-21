use std::cmp::max;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut missing_count = 0;
        for i in 0..minutes as usize {
            missing_count += customers[i] * grumpy[i];
        }

        let mut max_missing_count = missing_count;
        for i in 1..customers.len() {
            if i + minutes as usize - 1 >= customers.len() {
                break;
            }
            missing_count -= customers[i - 1] * grumpy[i - 1];
            missing_count += customers[i + minutes as usize - 1] * grumpy[i + minutes as usize - 1];
            max_missing_count = max(max_missing_count, missing_count);
        }

        let mut ans = 0;
        for i in 0..customers.len() {
            ans += customers[i] * (1 - grumpy[i]);
        }

        ans + max_missing_count
    }
}

struct Solution;
