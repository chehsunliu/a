impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answers = vec![];

        for i in 0..temperatures.len() {
            let mut answer = None;
            for j in (i + 1)..temperatures.len() {
                if temperatures[j] > temperatures[i] {
                    answer = Some(j - i);
                    break;
                }
            }
            answers.push(answer.unwrap_or(0) as i32);
        }

        answers
    }
}

struct Solution;
