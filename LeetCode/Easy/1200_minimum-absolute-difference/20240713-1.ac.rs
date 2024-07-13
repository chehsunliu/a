use std::cmp::min;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();

        let mut min_diff = i32::MAX;
        for i in 1..arr.len() {
            min_diff = min(min_diff, arr[i] - arr[i - 1]);
        }

        let mut ans = vec![];
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min_diff {
                ans.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
