impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut counts: Vec<Vec<(i32, i32)>> =
            vec![vec![(0, 0); grid[0].len() + 1]; grid.len() + 1];

        let mut count = 0;

        for r in 1..=grid.len() {
            let mut row_acc = (0, 0);
            for c in 1..=grid[0].len() {
                if grid[r - 1][c - 1] == 'X' {
                    row_acc = (row_acc.0 + 1, row_acc.1);
                } else if grid[r - 1][c - 1] == 'Y' {
                    row_acc = (row_acc.0, row_acc.1 + 1);
                }

                counts[r][c] = (counts[r - 1][c].0 + row_acc.0, counts[r - 1][c].1 + row_acc.1);

                if counts[r][c].0 == counts[r][c].1 && counts[r][c].0 > 0 {
                    count += 1;
                }
            }
        }
        
        // for r in 0..=grid.len() {
        //     for c in 0..=grid[0].len() {
        //         print!("{:?}", counts[r][c]);
        //     }
        //     println!();
        // }

        count
    }
}