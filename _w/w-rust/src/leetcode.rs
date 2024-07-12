use rand::random;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // 0 1 2 3 4 5 6
        select_kth_smallest(nums.clone(), nums.len() - k as usize)
    }
}

fn select_kth_smallest(mut nums: Vec<i32>, mut k: usize) -> i32 {
    // ... < m <= ...
    loop {
        let m = random::<usize>() % nums.len();

        let mut nums_l: Vec<i32> = vec![];
        let mut nums_r: Vec<i32> = vec![];
        let mut m_count = 0;

        for i in 0..nums.len() {
            if nums[i] > nums[m] {
                nums_r.push(nums[i]);
            } else if nums[i] < nums[m] {
                nums_l.push(nums[i]);
            } else {
                m_count += 1;
            }
        }

        if k < nums_l.len() {
            nums = nums_l;
        } else if k >= nums_l.len() + m_count {
            nums = nums_r;
            k = k - nums_l.len() - m_count;
        } else {
            return nums[m];
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
