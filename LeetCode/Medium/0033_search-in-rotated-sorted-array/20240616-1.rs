impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let k = Solution::f(&nums);
        
        let i1 = Solution::binary_search(&nums[..k], target);
        if i1 != -1 {
            return i1;
        }

        let i2 = Solution::binary_search(&nums[k..], target);
        if i2 != -1 {
            return k as i32 + i2;
        }
        
        -1
    }

    fn binary_search(nums: &[i32], target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;

        while l <= r {
            let m = (l + r) / 2;
            let v = nums[m as usize];

            if v == target {
                return m;
            } else if v > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        return -1;
    }

    fn f(nums: &[i32]) -> usize {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        let target = nums[l as usize];

        while l <= r {
            let m = (l + r) / 2;
            if nums[m as usize] < target {
                r = m - 1;
            } else if nums[m as usize] > target {
                l = m + 1;
            } else {
                l = m + 1;
            }
        }

        l as usize % nums.len()
    }
}

// 0 1 2 3 4 5 6
// 4 5 6 7 0 1 2
// 6 7 1 2 3 4 5 -> l = 2, r = 1
// 1 2 3 4 5 6 7 -> l = 7, r = 6

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
