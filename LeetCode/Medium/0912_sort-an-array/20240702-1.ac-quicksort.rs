use rand::Rng;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        f(&mut nums);
        nums
    }
}

fn f(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..nums.len());
    nums.swap(i, nums.len() - 1);

    let mut m = 0;
    let value = *nums.last().unwrap();
    for i in 0..(nums.len() - 1) {
        if nums[i] < value {
            nums.swap(i, m);
            m += 1;
        }
    }
    nums.swap(m, nums.len() - 1);

    f(&mut nums[..m]);
    f(&mut nums[m + 1..]);
}

// 1 2 4 5 4 9 3

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sort_array(vec![4, 1, 2, 3]), vec![1, 2, 3, 4]);
    }
}
