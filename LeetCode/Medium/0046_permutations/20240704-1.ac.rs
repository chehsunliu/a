use std::collections::HashSet;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = vec![];

        f(&mut HashSet::new(), &nums, &mut vec![], &mut answers);

        answers
    }
}

fn f(
    indexes: &mut HashSet<usize>,
    nums: &[i32],
    answer: &mut Vec<i32>,
    answers: &mut Vec<Vec<i32>>,
) {
    if answer.len() == nums.len() {
        answers.push(answer.clone());
        return;
    }

    for i in 0..nums.len() {
        if !indexes.contains(&i) {
            indexes.insert(i);
            answer.push(nums[i]);

            f(indexes, nums, answer, answers);

            indexes.remove(&i);
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
