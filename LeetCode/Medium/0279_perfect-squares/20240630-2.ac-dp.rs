use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let ks = (1..((n as f64).sqrt() as i32) + 1).collect::<Vec<i32>>();
        let mut reminder: HashMap<i32, i32> = HashMap::new();

        f(n, &ks, &mut reminder)
    }
}

fn f(n: i32, ks: &Vec<i32>, reminder: &mut HashMap<i32, i32>) -> i32 {
    if n == 1 || n == 0 {
        return n;
    } else if n < 0 {
        return 1000000;
    } else if reminder.contains_key(&n) {
        return *reminder.get(&n).unwrap();
    }

    let v = ks
        .iter()
        .map(|k| 1 + f(n - k * k, ks, reminder))
        .min()
        .unwrap();

    reminder.insert(n, v);
    v
}

struct Solution;

// f(n) = min{ f(n - 1^2) + 1, f(n - 2^2) + 1, ... , f(n - k^2) + 1 }
// f(1) = 1
// f(x) = 0 | x < 0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
        assert_eq!(Solution::num_squares(513), 3);
        assert_eq!(Solution::num_squares(1513), 2);
        assert_eq!(Solution::num_squares(2513), 3);
    }
}
