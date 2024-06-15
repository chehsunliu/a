impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        let mut ans = nums.clone();
        for i in 0..nums.len() {
            ans[(i + k as usize) % nums.len()] = nums[i];
        }
        for i in 0..nums.len() {
            nums[i] = ans[i];
        }
    }
}

struct Solution;
