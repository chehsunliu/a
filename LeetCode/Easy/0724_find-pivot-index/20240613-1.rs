impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        for v in &nums {
            sum += v;
        }

        let mut sum_l = 0;
        let mut sum_r = sum;

        for (i, v) in nums.iter().enumerate() {
            sum_r -= v;

            if sum_l == sum_r {
                return i as i32;
            }

            sum_l += v;
        }

        -1
    }
}

struct Solution;
