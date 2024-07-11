use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = -100000;
        let mut prev = -100000;

        for v in nums {
            prev = max(prev + v, v);
            ans = max(ans, prev);
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
