use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<i32> = HashSet::from_iter(nums1);
        let s2: HashSet<i32> = HashSet::from_iter(nums2);

        let ans = s1.intersection(&s2);
        ans.map(|&v| v).collect::<Vec<i32>>()
    }
}

struct Solution;
