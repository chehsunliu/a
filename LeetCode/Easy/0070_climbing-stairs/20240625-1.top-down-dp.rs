use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        f(n, &mut HashMap::new())
    }
}

fn f(n: i32, reminder: &mut HashMap<i32, i32>) -> i32 {
    if n == 0 {
        return 1;
    } else if n < 0 {
        return 0;
    }

    if reminder.contains_key(&n) {
        return *reminder.get(&n).unwrap();
    }

    let v = f(n - 1, reminder) + f(n - 2, reminder);
    reminder.insert(n, v);
    v
}

struct Solution;
