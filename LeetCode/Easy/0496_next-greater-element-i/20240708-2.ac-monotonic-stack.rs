use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut reminder: HashMap<i32, i32> = HashMap::new();
        let mut stack: VecDeque<i32> = VecDeque::new();

        for i in (0..nums2.len()).rev() {
            let current_v = nums2[i];

            while let Some(&previous_v) = stack.back() {
                if previous_v > current_v {
                    break;
                }

                stack.pop_back();
            }

            if let Some(&previous_v) = stack.back() {
                reminder.insert(current_v, previous_v);
            }
            stack.push_back(current_v);
        }

        nums1
            .into_iter()
            .map(|v| *reminder.get(&v).unwrap_or(&-1))
            .collect()
    }
}

// 4 6 3 1 2 5

// 5 2 1 3 6 4

// 5
// 5(0)

// 5 2
// 5(0) 2(1)

// 5 2 1
// 5(0) 2(1) 1(2)

// 5 2 1 3
// pop 1(2)
// pop 2(1)
// 5(0) 3(3)

// 5 2 1 3 6
// pop 3(3)
// pop 5(0)
// 6(4)

// 5 2 1 3 6 4
// 6(4) 4(5)

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
