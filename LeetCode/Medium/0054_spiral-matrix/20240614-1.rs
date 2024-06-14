impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut pos: (i32, i32) = (0, 0);

        let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut offset_index = 0;

        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;

        let mut used: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];

        let mut steps = 1;
        ans.push(matrix[pos.0 as usize][pos.1 as usize]);
        used[0][0] = true;

        while steps < m * n {
            let offset = offsets[offset_index];
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if new_pos.0 < 0
                || new_pos.0 >= m
                || new_pos.1 < 0
                || new_pos.1 >= n
                || used[new_pos.0 as usize][new_pos.1 as usize]
            {
                offset_index = (offset_index + 1) % offsets.len();
                continue;
            }

            pos = new_pos;
            ans.push(matrix[pos.0 as usize][pos.1 as usize]);
            used[pos.0 as usize][pos.1 as usize] = true;
            steps += 1;
        }

        ans
    }
}

struct Solution;
