use std::collections::HashMap;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let n = n as i64;

        if x == 0.0 || x == 1.0 {
            x
        } else if n == 0 {
            1.0
        } else if n > 0 {
            my_real_pow(x, n, &mut HashMap::new())
        } else {
            my_real_pow(1.0 / x, -n, &mut HashMap::new())
        }
    }
}

fn my_real_pow(x: f64, n: i64, dp: &mut HashMap<i64, f64>) -> f64 {
    if n == 1 {
        return x;
    }

    if let Some(v) = dp.get(&n) {
        return *v;
    }

    let v = if n % 2 == 1 {
        my_real_pow(x, n / 2, dp) * my_real_pow(x, n / 2, dp) * x
    } else {
        my_real_pow(x, n / 2, dp) * my_real_pow(x, n / 2, dp)
    };

    dp.insert(n, v);
    v
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_pow(1.000000001, 500000000), 1.6487213344757174);
        assert_eq!(Solution::my_pow(2.000000000, -2147483648), 0.0);
    }
}
