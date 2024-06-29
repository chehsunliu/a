use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited.get(&(i as i32, j as i32)).is_some() || grid[i][j] == '0' {
                    continue;
                }

                dfs((i as i32, j as i32), &grid, &mut visited);
                count += 1;
            }
        }

        count
    }
}

fn dfs((i, j): (i32, i32), grid: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    if i < 0
        || i >= m
        || j < 0
        || j >= n
        || grid[i as usize][j as usize] == '0'
        || visited.get(&(i, j)).is_some()
    {
        return;
    }

    visited.insert((i, j));

    dfs((i + 1, j), grid, visited);
    dfs((i - 1, j), grid, visited);
    dfs((i, j + 1), grid, visited);
    dfs((i, j - 1), grid, visited);
}

struct Solution;
