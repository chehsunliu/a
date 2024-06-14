impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];

        for i in 0..num_rows as usize {
            let mut row = vec![1; i + 1];
            if i < 2 {
                ans.push(row);
                continue;
            }

            for j in 1..i {
                row[j] = ans[i - 1][j - 1] + ans[i - 1][j];
            }

            ans.push(row);
        }

        ans
    }
}

struct Solution;
