impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let n = n as i64;

        if x == 0.0 || x == 1.0 {
            x
        } else if n == 0 {
            1.0
        } else if n > 0 {
            my_real_pow(x, n)
        } else {
            my_real_pow(1.0 / x, -n)
        }
    }
}

fn my_real_pow(x: f64, mut n: i64) -> f64 {
    let mut v = 1.0;

    while n > 0 {
        let mut current_n = 1;
        let mut current_v = x;

        while current_n * 2 <= n {
            current_v *= current_v;
            current_n *= 2;
        }

        v *= current_v;
        n -= current_n;
    }

    v
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_pow(2.0, 1), 2.0);
        assert_eq!(Solution::my_pow(2.0, 3), 8.0);
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.0, 15), 32768.0);
        // assert_eq!(Solution::my_pow(1.000000001, 500000000), 1.6487213344757174);
        assert_eq!(Solution::my_pow(2.000000000, -2147483648), 0.0);
    }
}
