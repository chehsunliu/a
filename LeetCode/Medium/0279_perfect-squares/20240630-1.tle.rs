use std::cmp::min;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        f(n, 100, 0)
    }
}

fn f(n: i32, v: i32, count: i32) -> i32 {
    if n == 0 {
        return count;
    }

    if v == 1 {
        return count + n;
    }

    if v * v > n {
        return f(n, v - 1, count);
    }

    let count1 = f(n - v * v, v, count + 1);
    let count2 = f(n, v - 1, count);

    min(count1, count2)
}

struct Solution;

// 9 1 1 1
// 4 4 4
// 4 4 1 1 1 1
// 4 1 1 1 1 1 1 1 1
//
// 100^2 99^2 ... 3^2 2^2 1^2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
