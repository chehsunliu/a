impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let l = search_l(&nums, target);
        let r = search_r(&nums, target);

        if l >= nums.len() || nums[l] != target {
            vec![-1, -1]
        } else {
            vec![l as i32, r as i32 - 1]
        }
    }
}

fn search_l(nums: &[i32], target: i32) -> usize {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    l as usize
}

fn search_r(nums: &[i32], target: i32) -> usize {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] <= target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    l as usize
}

struct Solution;
