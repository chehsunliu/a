use rand::random;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 0 1 2 3 4 5 6
        select_kth_smallest(&nums, nums.len() - k as usize)
    }
}

fn select_kth_smallest(nums: &[i32], k: usize) -> i32 {
    // ... < m <= ...
    let m = random::<usize>() % nums.len();

    let mut nums_l: Vec<i32> = vec![];
    let mut nums_r: Vec<i32> = vec![];

    for i in 0..nums.len() {
        if i == m {
            continue;
        }

        if nums[i] >= nums[m] {
            nums_r.push(nums[i]);
        } else {
            nums_l.push(nums[i]);
        }
    }

    if k == nums_l.len() {
        nums[m]
    } else if k < nums_l.len() {
        select_kth_smallest(&nums_l, k)
    } else {
        select_kth_smallest(&nums_r, k - nums_l.len() - 1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
