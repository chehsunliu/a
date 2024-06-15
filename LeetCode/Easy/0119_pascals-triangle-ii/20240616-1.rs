impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }

        if row_index == 1 {
            return vec![1, 1];
        }

        let mut row0: Vec<i32> = vec![0; row_index as usize + 1];
        let mut row1: Vec<i32> = vec![0; row_index as usize + 1];

        row0[0] = 1;
        row1[0] = 1;
        row1[1] = 1;

        let mut rows = vec![&mut row0, &mut row1];
        for i in 2..(row_index as usize + 1) {
            rows.swap(0, 1);

            for j in 1..i {
                rows[1][j] = rows[0][j - 1] + rows[0][j];
            }
            rows[1][i] = 1;
        }

        if row_index % 2 == 0 {
            row0
        } else {
            row1
        }
    }
}

struct Solution;
