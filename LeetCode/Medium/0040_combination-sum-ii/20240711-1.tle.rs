use std::collections::HashMap;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut answers = HashMap::new();
        f(&candidates, 0, target, vec![], &mut answers);
        answers.into_values().collect()
    }
}

fn f(
    arr: &[i32],
    current: i32,
    target: i32,
    mut answer: Vec<i32>,
    answers: &mut HashMap<String, Vec<i32>>,
) {
    if current == target {
        answers.insert(
            answer
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(","),
            answer,
        );
        return;
    }

    if current > target || arr.is_empty() {
        return;
    }

    f(&arr[1..], current, target, answer.clone(), answers);

    answer.push(arr[0]);
    f(&arr[1..], current + arr[0], target, answer, answers);
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
