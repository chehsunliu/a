use std::iter::zip;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut items: Vec<(String, i32)> = zip(names, heights).collect();
        items.sort_by_key(|item| -item.1);

        items.into_iter().map(|item| item.0).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
