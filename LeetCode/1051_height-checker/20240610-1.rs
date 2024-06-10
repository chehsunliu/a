impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected_heights = heights.clone();
        expected_heights.sort();

        let mut ans = 0;

        for i in 0..heights.len() {
            ans += (expected_heights[i] != heights[i]) as i32;
        }

        ans
    }
}
