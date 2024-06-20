use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut reminder: HashMap<i32, i32> = HashMap::new();

        for num in &nums {
            reminder.insert(*num, 1 + *reminder.get(num).unwrap_or(&0));
        }

        for (k, v) in reminder {
            if v > 1 {
                return k;
            }
        }

        panic!("GG")
    }
}

struct Solution;
