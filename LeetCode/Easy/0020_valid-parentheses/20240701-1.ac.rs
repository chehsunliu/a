use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<char> = VecDeque::new();

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push_back(c),
                ')' => match stack.pop_back() {
                    Some('(') => {}
                    _ => return false,
                },
                '}' => match stack.pop_back() {
                    Some('{') => {}
                    _ => return false,
                },
                ']' => match stack.pop_back() {
                    Some('[') => {}
                    _ => return false,
                },
                _ => panic!("GG"),
            }
        }

        stack.is_empty()
    }
}

struct Solution;
