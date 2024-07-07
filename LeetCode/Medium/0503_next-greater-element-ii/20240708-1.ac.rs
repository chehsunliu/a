use std::collections::VecDeque;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![-1; nums.len()];
        let mut stack: VecDeque<usize> = VecDeque::new();

        for _ in 0..2 {
            for i in (0..nums.len()).rev() {
                let v = nums[i];

                while let Some(&prev_i) = stack.back() {
                    if nums[prev_i] > v {
                        break;
                    }

                    stack.pop_back();
                }

                if let Some(&prev_i) = stack.back() {
                    ans[i] = nums[prev_i];
                }
                stack.push_back(i);
            }
        }

        ans
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
    #[test]
    fn basic() {}
}
