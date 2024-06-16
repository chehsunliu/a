impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let mut l = 0;
        let mut r = x;

        while l <= r {
            let m = (l + r) / 2;
            let v: i64 = m * m;

            if v == x {
                return m as i32;
            } else if v < x {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        r as i32
    }
}

struct Solution;

// l = 0, r = 8, m = 4 | 16 > 8 | r = 3
// l = 0, r = 3, m = 1 |  1 < 8 | l = 2
// l = 2, r = 3, m = 2 |  4 < 8 | l = 3
// l = 3, r = 3, m = 3 |  9 > 8 | r = 2
// l = 3, r = 2 | end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(901), 30);
        assert_eq!(Solution::my_sqrt(9901), 99);
    }
}
