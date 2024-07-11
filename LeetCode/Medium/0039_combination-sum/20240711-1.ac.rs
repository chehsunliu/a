impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        f(&candidates, 0, target, vec![], &mut answers);
        answers
    }
}

fn f(arr: &[i32], current: i32, target: i32, mut answer: Vec<i32>, answers: &mut Vec<Vec<i32>>) {
    if current == target {
        answers.push(answer);
        return;
    }

    if current > target || arr.is_empty() {
        return;
    }

    f(&arr[1..], current, target, answer.clone(), answers);

    answer.push(arr[0]);
    f(arr, current + arr[0], target, answer, answers);
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
