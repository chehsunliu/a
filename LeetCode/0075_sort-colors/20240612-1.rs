impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];

        for num in nums.iter() {
            counts[*num as usize] += 1;
        }

        let mut offset = 0;
        for (i, count) in counts.iter().enumerate() {
            for j in 0..*count {
                nums[j + offset] = i as i32;
            }
            offset += count;
        }
    }
}

struct Solution;

// #[cfg(test)]
// mod tests {
//     use super::Solution;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(
//             Solution::relative_sort_array(
//                 vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
//                 vec![2, 1, 4, 3, 9, 6]
//             ),
//             vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
//         );
//     }
// }
