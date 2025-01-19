use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (c1, c2) = (Self::f(s), Self::f(t));

        let (k1, k2): (HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(c1.keys().map(|c| *c)),
            HashSet::from_iter(c2.keys().map(|c| *c)),
        );

        if k1.len() != k2.len() || !k1.is_subset(&k2) {
            return false;
        }

        for c in k1.iter() {
            if c1.get(c).unwrap() != c2.get(c).unwrap() {
                return false;
            }
        }

        true
    }

    fn f(s: String) -> HashMap<char, i32> {
        let mut r = HashMap::new();

        for c in s.chars() {
            r.insert(c, r.get(&c).unwrap_or(&0) + 1);
        }

        r
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
