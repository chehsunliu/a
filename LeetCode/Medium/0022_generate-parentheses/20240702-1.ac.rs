impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answers = Vec::new();
        f(n, 0, String::new(), &mut answers);
        answers
    }
}

fn f(n: i32, stack_count: i32, mut ans: String, answers: &mut Vec<String>) {
    if n == 0 {
        ans.push_str(&")".repeat(stack_count as usize));
        answers.push(ans);
        return;
    }

    for i in 0..=stack_count {
        let mut new_ans = ans.clone();
        new_ans.push_str(&")".repeat(i as usize));
        new_ans.push('(');
        f(n - 1, stack_count - i + 1, new_ans, answers);
    }
}

// ()
// ()() (())
// ()()() ()(()) (())() (()()) ((()))

// (
//  ()
//   ()(
//    ()((
//     ()(())
//    ()()
//     ()()()
// ((
//  (((
//  (()

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        println!("{:?}", Solution::generate_parenthesis(3));
    }
}
