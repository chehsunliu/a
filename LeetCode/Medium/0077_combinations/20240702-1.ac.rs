impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut answers = Vec::new();
        f(Vec::new(), &mut answers, n, k);
        answers
    }
}

fn f(answer: Vec<i32>, answers: &mut Vec<Vec<i32>>, n: i32, k: i32) {
    if k == 0 {
        answers.push(answer);
        return;
    }

    let x = *answer.last().unwrap_or(&0);
    for x2 in (x + 1)..=n {
        let mut next_answer = answer.clone();
        next_answer.push(x2);
        f(next_answer, answers, n, k - 1);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
