impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

struct Solution;
