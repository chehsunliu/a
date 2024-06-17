impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut l = 0;
        let mut r = (c as f64).sqrt() as i64;

        while l <= r {
            let s = l * l + r * r;
            if s == c {
                return true;
            }

            if s > c {
                r -= 1;
            } else {
                l += 1;
            }
        }

        false
    }
}

struct Solution;
