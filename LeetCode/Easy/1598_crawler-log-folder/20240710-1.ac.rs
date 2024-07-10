use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();

        for log in logs {
            match log.as_str() {
                "../" => {
                    stack.pop_back();
                }
                "./" => {}
                _ => {
                    stack.push_back(0);
                }
            }
        }

        stack.len() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
