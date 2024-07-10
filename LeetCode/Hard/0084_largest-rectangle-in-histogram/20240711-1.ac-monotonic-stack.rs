use std::cmp::max;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();

        if *heights.last().unwrap() != 0 {
            heights.push(0);
        }

        for i in 0..heights.len() {
            let mut count = 0;
            while let Some(&(i0, prev_count)) = stack.back() {
                if heights[i0] <= heights[i] {
                    break;
                }

                ans = max(ans, heights[i0] * (i - i0 + prev_count) as i32);
                stack.pop_back();
                count += 1 + prev_count;
            }

            stack.push_back((i, count));
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
