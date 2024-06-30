use std::collections::VecDeque;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();

        for token in &tokens {
            match token.as_str() {
                "+" => {
                    let (v2, v1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                    stack.push_back(v1 + v2);
                }
                "-" => {
                    let (v2, v1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                    stack.push_back(v1 - v2);
                }
                "*" => {
                    let (v2, v1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                    stack.push_back(v1 * v2);
                }
                "/" => {
                    let (v2, v1) = (stack.pop_back().unwrap(), stack.pop_back().unwrap());
                    stack.push_back(v1 / v2);
                }
                _ => {
                    stack.push_back(token.parse::<i32>().unwrap());
                }
            }
        }

        *stack.back().unwrap()
    }
}

struct Solution;
