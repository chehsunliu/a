use std::collections::VecDeque;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answers = Vec::new();

        let mut queue: VecDeque<(i32, i32, String)> = VecDeque::new();
        queue.push_back((n, 0, String::new()));

        while let Some((n, stack_count, mut ans)) = queue.pop_front() {
            if n == 0 {
                ans.push_str(&")".repeat(stack_count as usize));
                answers.push(ans);
                continue;
            }

            for i in 0..=stack_count {
                let mut new_ans = ans.clone();
                new_ans.push_str(&")".repeat(i as usize));
                new_ans.push('(');
                queue.push_back((n - 1, stack_count - i + 1, new_ans));
            }
        }

        answers
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        println!("{:?}", Solution::generate_parenthesis(3));
    }
}
