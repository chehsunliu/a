impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let mut ans = 0;
        for i in (0..nums.len()).step_by(2) {
            ans += nums[i];
        }

        ans
    }
}

struct Solution;
