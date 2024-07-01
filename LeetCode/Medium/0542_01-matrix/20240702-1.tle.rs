use std::cmp::min;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;

        let mut ans = vec![vec![i32::MAX; n as usize]; m as usize];

        for i in 0..m {
            for j in 0..n {
                if mat[i as usize][j as usize] != 0 {
                    continue;
                }

                let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
                let mut visited: HashSet<(i32, i32)> = HashSet::new();

                queue.push_back((i, j, 0));

                while let Some((i2, j2, d)) = queue.pop_front() {
                    if i2 < 0 || i2 >= m || j2 < 0 || j2 >= n || visited.contains(&(i2, j2)) {
                        continue;
                    }

                    visited.insert((i2, j2));
                    ans[i2 as usize][j2 as usize] = min(ans[i2 as usize][j2 as usize], d);

                    queue.push_back((i2 + 1, j2, d + 1));
                    queue.push_back((i2 - 1, j2, d + 1));
                    queue.push_back((i2, j2 + 1, d + 1));
                    queue.push_back((i2, j2 - 1, d + 1));
                }
            }
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
