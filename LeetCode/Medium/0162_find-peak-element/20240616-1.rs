impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32;

        while l <= r {
            let m = l + (r - l) / 2;
            let v = nums[m as usize];

            if nums.len() == 1 {
                return m;
            } else if m == 0 {
                if v > nums[m as usize + 1] {
                    return m;
                }

                l = m + 1;
            } else if m == nums.len() as i32 - 1 {
                if v > nums[m as usize - 1] {
                    return m;
                }

                r = m - 1;
            } else {
                if v > nums[m as usize + 1] && v > nums[m as usize - 1] {
                    return m;
                }

                if v < nums[m as usize + 1] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }

        l
    }
}

struct Solution;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn basic() {
//         assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
//         assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
//         assert_eq!(Solution::search(vec![1], 0), -1);
//     }
// }
