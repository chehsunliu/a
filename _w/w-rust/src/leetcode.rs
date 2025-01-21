use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for v in &nums {
            counter.insert(*v, counter.get(v).unwrap_or(&0) + 1);
        }

        let mut counter = counter
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(i32, i32)>>();
        counter.sort_by(|&c1, &c2| c1.1.cmp(&c2.1).reverse());

        counter[..k as usize].iter().map(|(k, _)| *k).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
