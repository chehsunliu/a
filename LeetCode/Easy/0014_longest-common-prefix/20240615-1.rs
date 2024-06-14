use std::cmp::min;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();

        let len = strs
            .iter()
            .map(|s| s.len())
            .fold(usize::MAX as usize, |x, acc| min(x, acc));

        for i in 0..len {
            let mut same = true;

            let c1 = strs[0].as_bytes()[i];
            for s in &strs {
                let c2 = s.as_bytes()[i];
                if c1 != c2 {
                    same = false;
                    break;
                }
            }

            if !same {
                break;
            }

            ans.push(c1 as char);
        }

        ans
    }
}

struct Solution;
