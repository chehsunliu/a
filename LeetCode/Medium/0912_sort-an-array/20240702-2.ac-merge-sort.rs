impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        merge_sort(&mut nums);
        nums
    }
}

fn merge_sort(nums: &mut [i32]) {
    if nums.len() == 1 {
        return;
    }

    let m = nums.len() / 2;
    merge_sort(&mut nums[..m]);
    merge_sort(&mut nums[m..]);

    merge(nums, m);
}

fn merge(nums: &mut [i32], m: usize) {
    let mut new_nums: Vec<i32> = Vec::with_capacity(nums.len());

    let nums1 = &nums[..m];
    let nums2 = &nums[m..];

    let (mut i1, mut i2) = (0, 0);
    while i1 < nums1.len() && i2 < nums2.len() {
        if nums1[i1] < nums2[i2] {
            new_nums.push(nums1[i1]);
            i1 += 1;
        } else {
            new_nums.push(nums2[i2]);
            i2 += 1;
        }
    }

    while i1 < nums1.len() {
        new_nums.push(nums1[i1]);
        i1 += 1;
    }

    while i2 < nums2.len() {
        new_nums.push(nums2[i2]);
        i2 += 1;
    }

    for i in 0..nums.len() {
        nums[i] = new_nums[i];
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sort_array(vec![4, 1, 2, 3]), vec![1, 2, 3, 4]);
    }
}
