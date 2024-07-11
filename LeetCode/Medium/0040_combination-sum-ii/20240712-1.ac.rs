impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut counts = [0; 51];
        for v in candidates {
            counts[v as usize] += 1;
        }

        let mut answers = vec![];
        f(1, &counts, 0, target, vec![], &mut answers);

        answers
    }
}

fn f(
    index: usize,
    counts: &[i32],
    current: i32,
    target: i32,
    mut answer: Vec<i32>,
    answers: &mut Vec<Vec<i32>>,
) {
    if current == target {
        answers.push(answer);
        return;
    }

    if current > target || index >= counts.len() {
        return;
    }

    let count = counts[index];

    for i in 0..=count {
        f(
            index + 1,
            counts,
            current + index as i32 * i,
            target,
            answer.clone(),
            answers,
        );
        answer.push(index as i32);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
