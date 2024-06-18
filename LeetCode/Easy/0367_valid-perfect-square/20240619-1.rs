impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut l = 1;
        let mut r = num as i64;

        while l <= r {
            let m = l + (r - l) / 2;
            let v = m * m;

            if num == v {
                return true;
            } else if num < v {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        false
    }
}

struct Solution;
