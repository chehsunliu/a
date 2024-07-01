impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        f(&nums, target)
    }
}

fn f(nums: &[i32], target: i32) -> i32 {
    if nums.len() == 0 {
        return if target == 0 { 1 } else { 0 };
    }

    f(&nums[1..], target + nums[0]) + f(&nums[1..], target - nums[0])
}

struct Solution;
