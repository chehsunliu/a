impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let v = nums[m as usize];

            if v >= nums[0] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        nums[l as usize % nums.len()]
    }
}

struct Solution;
