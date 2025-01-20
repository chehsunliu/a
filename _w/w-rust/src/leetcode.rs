impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        f(s.as_bytes(), t.as_bytes())
    }
}

fn f(s: &[u8], t: &[u8]) -> bool {
    if s.is_empty() {
        return true;
    }

    if t.is_empty() {
        return false;
    }

    if s[0] == t[0] {
        f(&s[1..], &t[1..])
    } else {
        f(s, &t[1..])
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
