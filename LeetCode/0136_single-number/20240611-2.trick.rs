impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for a in &nums {
            ans = ans ^ *a;
        }

        ans
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
