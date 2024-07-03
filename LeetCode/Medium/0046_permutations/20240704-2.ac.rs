impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = vec![];

        f(&nums, &mut vec![], &mut answers);

        answers
    }
}

fn f(nums: &[i32], answer: &mut Vec<i32>, answers: &mut Vec<Vec<i32>>) {
    if answer.len() == nums.len() {
        answers.push(answer.clone());
        return;
    }

    for i in 0..nums.len() {
        if !answer.contains(&nums[i]) {
            answer.push(nums[i]);
            f(nums, answer, answers);
            answer.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
