use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        f(&heights, (0, heights.len() as i32 - 1))
    }
}

fn f(heights: &[i32], (l, r): (i32, i32)) -> i32 {
    if l > r {
        return 0;
    } else if l == r {
        return heights[l as usize];
    }

    let mut m = l;
    for i in l..=r {
        if heights[i as usize] < heights[m as usize] {
            m = i;
        }
    }

    let mut ans = (r - l + 1) * heights[m as usize];
    ans = max(ans, f(heights, (l, m - 1)));
    ans = max(ans, f(heights, (m + 1, r)));
    ans
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
