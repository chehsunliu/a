// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while l <= r {
            let m = l + (r - l) / 2;
            if self.isBadVersion(m) {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        l
    }

    fn isBadVersion(&self, version: i32) -> bool {
        unimplemented!()
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
