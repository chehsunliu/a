impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];

        let m = mat.len() as i32;
        let n = mat[0].len() as i32;

        for s in 0..m + n - 1 {
            if s % 2 == 0 {
                let mut i: i32 = s;
                let mut j: i32 = 0;

                if i >= m {
                    j += i - m + 1;
                    i = m - 1;
                }

                while i >= 0 && j < n {
                    ans.push(mat[i as usize][j as usize]);
                    i -= 1;
                    j += 1;
                }
            } else {
                let mut i = 0;
                let mut j = s;

                if j >= n {
                    i += j - n + 1;
                    j = n - 1;
                }

                while i < m && j >= 0 {
                    ans.push(mat[i as usize][j as usize]);
                    i += 1;
                    j -= 1;
                }
            }
        }

        ans
    }
}

// 0 1 2 3 4
// 1 2 3 4 5
// 2 3 4 5 6
// - - - - -
// - - - - -

// 0 1 -
// 1 2 -
// 2 3 -

struct Solution;
