impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let mut min_moves = 0;

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                let moves = nums[i - 1] + 1 - nums[i];
                nums[i] += moves;
                min_moves += moves;
            }
        }

        min_moves
    }
}

struct Solution;
