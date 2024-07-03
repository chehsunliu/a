const DIGITS: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let digits = digits
            .chars()
            .map(|c| c as usize - '2' as usize)
            .collect::<Vec<usize>>();

        let mut answers = vec![];
        f(&digits, &mut String::new(), &mut answers);
        answers
    }
}

fn f(digits: &[usize], answer: &mut String, answers: &mut Vec<String>) {
    if digits.is_empty() {
        answers.push(answer.clone());
        return;
    }

    let chars = DIGITS[digits[0]];
    for c in chars.chars() {
        answer.push(c);
        f(&digits[1..], answer, answers);
        answer.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
