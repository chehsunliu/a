impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let m = (l + r) / 2;
            let v = nums[m as usize];

            if v == target {
                return m;
            } else if v > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        -1
    }
}

struct Solution;
