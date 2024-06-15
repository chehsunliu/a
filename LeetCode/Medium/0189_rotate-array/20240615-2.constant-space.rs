impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        nums.reverse();
        nums[..k as usize].reverse();
        nums[k as usize..].reverse();
    }
}

struct Solution;
