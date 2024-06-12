impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut j = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[j] {
                j = i;
            }
        }

        let mut is_valid = true;
        for i in 0..nums.len() {
            if i != j {
                is_valid &= nums[j] >= 2 * nums[i];
            }
        }

        if is_valid {
            j as i32
        } else {
            -1
        }
    }
}

struct Solution;
