use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for v in &nums {
            let count = counts.get(v).unwrap_or(&0);
            counts.insert(*v, count + 1);
        }

        for (v, count) in counts.iter() {
            if *count == 1 {
                return *v;
            }
        }

        panic!("no answer!?");
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     Solution::relative_sort_array(
        //         vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
        //         vec![2, 1, 4, 3, 9, 6]
        //     ),
        //     vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        // );
    }
}
