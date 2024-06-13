fn solution(s: &str) -> String {
    let mut ans = String::new();

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            ans.push(' ');
        }
        ans.push(c);
    }

    ans
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
