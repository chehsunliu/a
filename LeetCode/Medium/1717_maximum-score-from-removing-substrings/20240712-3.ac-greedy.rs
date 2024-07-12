use std::cmp::{max, min};

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let chunks = create_chunks(&s);

        let mut ans = 0;
        for chunk in chunks {
            ans += handle_chunk(chunk, (x, y));
        }

        ans
    }
}

fn handle_chunk(chunk: String, (x, y): (i32, i32)) -> i32 {
    let mut points = 0;
    let mut stack = "".to_string();
    let token0 = if x > y { "ab" } else { "ba" };

    let min_points = min(x, y);
    let max_points = max(x, y);

    for c in chunk.chars() {
        stack.push(c);
        if stack.len() >= 2 && &stack[(stack.len() - 2)..stack.len()] == token0 {
            stack.pop();
            stack.pop();
            points += max_points;
        }
    }

    let mut counts = (0, 0);
    for c in stack.chars() {
        if c == 'a' {
            counts = (counts.0 + 1, counts.1)
        } else {
            counts = (counts.0, counts.1 + 1)
        }
    }

    points + min(counts.0, counts.1) * min_points
}

fn create_chunks(s: &str) -> Vec<String> {
    let mut chunks: Vec<String> = vec![];
    let mut chunk = "".to_string();
    for c in s.chars() {
        if c == 'a' || c == 'b' {
            chunk.push(c);
        } else {
            if !chunk.is_empty() {
                chunks.push(chunk.clone());
            }
            chunk.clear();
        }
    }
    if !chunk.is_empty() {
        chunks.push(chunk.clone());
    }
    chunks
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
        assert_eq!(
            Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4),
            20
        );
        assert_eq!(
            Solution::maximum_gain("aaababaaabababaaabaaabbbbbbbbabababaaa".to_string(), 4, 5),
            80
        );
    }
}

// aaababaaabababaaabaaabbbbbbbbabababaaa
// aaaaaaaaabbbbb

// abab
// > ba ab
// > ab ab

// baba
// > ba ba
// > ab ba

// ababa
// ab ab
// ab ba
// ba ba
