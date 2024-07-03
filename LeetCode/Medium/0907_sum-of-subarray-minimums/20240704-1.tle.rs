use std::cmp::min;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        
        for i in 0..arr.len() {
            let mut sub_ans = i32::MAX;
            for j in i..arr.len() {
                sub_ans = min(sub_ans, arr[j]);
                ans += sub_ans;
                ans %= 1_000_000_007;
            }
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
