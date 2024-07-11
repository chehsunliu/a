impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ans = String::new();

        for c in s.chars() {
            if c != ')' {
                ans.push(c);
                continue;
            }

            let mut tmp = String::new();
            while let Some(c) = ans.pop() {
                if c != '(' {
                    tmp.push(c);
                } else {
                    break;
                }
            }
            ans.push_str(&tmp);
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
