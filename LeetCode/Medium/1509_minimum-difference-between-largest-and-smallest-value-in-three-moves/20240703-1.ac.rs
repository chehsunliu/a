use std::cmp::min;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        if nums.len() <= 4 {
            return 0;
        }

        let (mut l, mut r) = (0, nums.len() - 4);

        let mut ans = i32::MAX;

        for _ in 0..4 {
            ans = min(ans, nums[r] - nums[l]);
            r += 1;
            l += 1
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
