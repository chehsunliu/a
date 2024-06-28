use std::collections::HashMap;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut counts: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            counts.insert(c, 1 + *counts.get(&c).unwrap_or(&0));
        }

        let mut odds = 0;
        let mut evens = 0;

        for (_, &count) in &counts {
            if count % 2 == 0 {
                evens += 1;
            } else {
                odds += 1;
            }
        }

        if s.len() % 2 == 0 {
            odds == 0
        } else {
            odds == 1
        }
    }
}

struct Solution;
