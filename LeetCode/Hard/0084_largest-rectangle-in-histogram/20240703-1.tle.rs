use std::cmp::{max, min};

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut area = 0;

        for i in 0..heights.len() {
            let mut h = i32::MAX;
            for j in i..heights.len() {
                h = min(h, heights[j]);
                area = max(area, h * (j as i32 + 1 - i as i32));
            }
        }

        area
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
