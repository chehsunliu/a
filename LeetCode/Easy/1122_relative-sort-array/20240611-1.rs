use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut distinct_numbers: HashSet<i32> = HashSet::new();
        for a in &arr2 {
            distinct_numbers.insert(*a);
        }

        let mut counts: HashMap<i32, i32> = HashMap::new();
        for a in &arr1 {
            let count = counts.get(a).unwrap_or(&0);
            counts.insert(*a, *count + 1);
        }

        let mut ans1: Vec<i32> = vec![];
        let mut ans2: Vec<i32> = vec![];

        for a in &arr2 {
            let count = counts.get(a).unwrap_or(&0);
            for _ in 0..*count {
                ans1.push(*a);
            }
        }

        for a in &arr1 {
            if !distinct_numbers.contains(a) {
                ans2.push(*a);
            }
        }

        ans2.sort();
        for a in &ans2 {
            ans1.push(*a);
        }

        ans1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }
}
