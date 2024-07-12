use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let chunks = create_chunks(&s);
        let mut dp: HashMap<String, i32> = HashMap::new();

        let mut ans = 0;
        for chunk in chunks {
            ans += handle_chunk(chunk, 0, (x, y), &mut dp);
        }

        ans
    }
}

fn handle_chunk(
    chunk: String,
    points: i32,
    (x, y): (i32, i32),
    dp: &mut HashMap<String, i32>,
) -> i32 {
    if chunk.len() <= 1 {
        return points;
    }

    if dp.contains_key(&chunk) {
        return points + dp.get(&chunk).unwrap();
    }

    let mut acc_points = points;

    for i in 0..(chunk.len() - 1) {
        let token = &chunk[i..(i + 2)];
        if token != "ab" && token != "ba" {
            continue;
        }

        let token_points = if token == "ab" { x } else { y };

        let mut new_chunk = "".to_string();
        new_chunk.push_str(&chunk[0..i]);
        new_chunk.push_str(&chunk[(i + 2)..chunk.len()]);
        acc_points = max(
            acc_points,
            handle_chunk(new_chunk, points + token_points, (x, y), dp),
        );
    }

    dp.insert(chunk, acc_points - points);

    acc_points
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
    }
}
