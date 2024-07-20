use std::cmp::min;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; col_sum.len()]; row_sum.len()];
        
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                ans[i][j] = min(row_sum[i], col_sum[j]);
                row_sum[i] -= ans[i][j];
                col_sum[j] -= ans[i][j];
            }
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}

//  5 5 0 0
//  7 3 4 0
// 10 0 2 8
//    8 6 8
