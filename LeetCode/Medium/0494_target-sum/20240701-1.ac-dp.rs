use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: HashMap<(usize, i32), i32> = HashMap::new();
        f(&nums, 0, target, &mut dp)
    }
}

fn f(nums: &[i32], i: usize, target: i32, dp: &mut HashMap<(usize, i32), i32>) -> i32 {
    if i == nums.len() {
        return if target == 0 { 1 } else { 0 };
    }

    if dp.contains_key(&(i, target)) {
        return *dp.get(&(i, target)).unwrap();
    }

    let count = f(nums, i + 1, target + nums[i], dp) + f(nums, i + 1, target - nums[i], dp);
    dp.insert((i, target), count);
    count
}

struct Solution;
