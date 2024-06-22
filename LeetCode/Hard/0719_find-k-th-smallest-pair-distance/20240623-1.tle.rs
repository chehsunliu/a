impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut ds = vec![];
        for i in 0..(nums.len()-1) {
            for j in (i + 1)..nums.len() {
                ds.push((nums[i] - nums[j]).abs());
            }
        }
        ds.sort();
        ds[k as usize - 1]
    }
}

struct Solution;
