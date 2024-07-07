use std::collections::VecDeque;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut ans: u64 = 0;
        let mut stack: VecDeque<usize> = VecDeque::new();

        for i in 0..arr.len() {
            while let Some(&j) = stack.back() {
                if arr[j] <= arr[i] {
                    break;
                }

                // k < j < i
                stack.pop_back();
                let r1 = if let Some(&k) = stack.back() {
                    j - k
                } else {
                    j + 1
                };
                let r2 = i - j;
                ans += r1 as u64 * r2 as u64 * arr[j] as u64;
                ans %= 1_000_000_007;
            }

            stack.push_back(i);
        }

        while let Some(j) = stack.pop_back() {
            let r1 = if let Some(&k) = stack.back() {
                j - k
            } else {
                j + 1
            };
            let r2 = arr.len() - j;
            ans += r1 as u64 * r2 as u64 * arr[j] as u64;
            ans %= 1_000_000_007;
        }

        ans as i32
    }
}

// 0 1 2 3
// 3 1 2 4 + 1

// 3
// 3(0)

// 3 1
// pop 3(0) [0~0~0] +3*1=3
// 1(1)

// 3 1 2
// 1(1) 2(2)

// 3 1 2 4
// 1(1) 2(2) 4(3)

// pop 4(3) [3~3~3] +4*1=4
// pop 2(2) [2~2~3] +2*2=4
// pop 1(1) [0~1~3] +1*6=6

// ----

// 0 1 2 3 4 5 6 7 8
// 0 3 4 5 2 3 4 1 4

// 0
// 0(0)

// 0 3
// 0(0) 3(1)

// 0 3 4
// 0(0) 3(1) 4(2)

// 0 3 4 5
// 0(0) 3(1) 4(2) 5(3)

// 0 3 4 5 2
// pop 5(3) [3~3~3] +5*1=5
// pop 4(2) [2~2~3] +4*2=8
// pop 3(1) [1~1~3] +3*3=9
// 0(0) 2(4)

// 0 3 4 5 2 3
// 0(0) 2(4) 3(5)

// 0 3 4 5 2 3 4
// 0(0) 2(4) 3(5) 4(6)

// 0 3 4 5 2 3 4 1
// pop 4(6) [6~6~6] +4*1=4
// pop 3(5) [5~5~6] +3*2=6
// pop 2(4) [1~4~6] +2*12=24
// 0(0) 1(7)

// 0 3 4 5 2 3 4 1 4
// 0(0) 1(7) 4(8)
// pop 4(8) [8~8~8] +4*1=4
// pop 1(7) [1~7~8] +1*14=14
// pop 0(0) [0~0~8] +0*9=0

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
        assert_eq!(
            Solution::sum_subarray_mins(vec![3, 4, 5, 2, 3, 4, 1, 4]),
            74
        );
    }
}
