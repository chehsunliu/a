use std::collections::HashMap;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut reminder = HashMap::new();
        return fib(n, &mut reminder);
    }
}

fn fib(n: i32, reminder: &mut HashMap<i32, i32>) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    if reminder.contains_key(&n) {
        return *reminder.get(&n).unwrap();
    }

    let v = fib(n - 1, reminder) + fib(n - 2, reminder);
    reminder.insert(n, v);
    v
}

struct Solution;
