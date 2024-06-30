use std::collections::VecDeque;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answers = vec![0; temperatures.len()];
        let mut stack: VecDeque<usize> = VecDeque::new();

        for i in 0..(temperatures.len() - 1) {
            if temperatures[i] < temperatures[i + 1] {
                answers[i] = 1;
                while stack
                    .back()
                    .filter(|&&j| temperatures[j] < temperatures[i + 1])
                    .is_some()
                {
                    let j = stack.pop_back().unwrap();
                    answers[j] = (i + 1 - j) as i32;
                }
            } else {
                stack.push_back(i);
            }
        }

        answers
    }
}

struct Solution;

// 73 74 75 71 69 72 76 73
//  ^ stack = []
//     ^ stack = []
//        ^ stack = [75]
//           ^ stack = [75, 71]
//              ^ stack = [75, 71]
