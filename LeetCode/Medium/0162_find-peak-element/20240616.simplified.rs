impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            let v = get_value(&nums, m);
            let v_l = get_value(&nums, m - 1);
            let v_r = get_value(&nums, m + 1);

            if v > v_r && v > v_l {
                return m;
            } else if v < v_r {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        l
    }
}

fn get_value(nums: &[i32], i: i32) -> i64 {
    if i < 0 || i >= nums.len() as i32 {
        i64::MIN
    } else {
        nums[i as usize] as i64
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
