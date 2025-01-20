use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup: HashMap<i32, usize> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            lookup.insert(*v, i);
        }

        for (i, v) in nums.iter().enumerate() {
            if let Some(j) = lookup.get(&(target - *v)) {
                if i != *j {
                    return vec![i as i32, *j as i32];
                }
            }
        }

        unreachable!()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
