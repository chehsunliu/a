use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut mem: HashMap<u8, i32> = HashMap::new();

        for c in s.as_bytes() {
            let v = match mem.get(c) {
                Some(v) => *v,
                None => 0,
            };
            mem.insert(*c, v + 1);
        }

        let mut has_odd = false;
        let mut ans = 0;

        for count in mem.values() {
            ans += count - count % 2;
            has_odd |= count % 2 == 1
        }

        ans + has_odd as i32
    }
}
