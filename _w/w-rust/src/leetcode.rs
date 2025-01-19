use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        nums.iter().for_each(|v| {
            if let Some(count) = counts.get_mut(v) {
                *count += 1;
            } else {
                counts.insert(*v, 1);
            }
        });

        counts.values().any(|count| *count >= 2)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
