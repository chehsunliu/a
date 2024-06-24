impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        if nums.iter().sum::<i32>() == nums.len() as i32 {
            return 0;
        }

        let mut nums = nums.clone();
        let mut l = 0;
        let k = k as usize;
        let mut flips = 0;

        while true {
            while l < nums.len() && nums[l] != 0 {
                l += 1;
            }

            if l + k - 1 >= nums.len() {
                break;
            }

            for i in l..(l + k) {
                nums[i] = 1 - nums[i];
            }

            flips += 1;
        }

        for v in &nums {
            if *v == 0 {
                return -1;
            }
        }

        flips
    }
}

struct Solution;
