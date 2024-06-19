use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        let m1 = f(&nums1);
        let m2 = f(&nums2);

        for (v1, count1) in m1.iter() {
            if let Some(count2) = m2.get(v1) {
                for _ in 0..min(*count1, *count2) {
                    ans.push(*v1);
                }
            }
        }

        ans
    }
}

fn f(nums: &[i32]) -> HashMap<i32, i32> {
    let mut m = HashMap::new();
    for v in nums {
        m.insert(*v, 1 + *m.get(v).unwrap_or(&0));
    }
    m
}

struct Solution;
