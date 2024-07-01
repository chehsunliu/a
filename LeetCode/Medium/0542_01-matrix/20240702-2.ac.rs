use std::cmp::min;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;

        let mut ans = vec![vec![i32::MAX; n as usize]; m as usize];

        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        for i in 0..m {
            for j in 0..n {
                if mat[i as usize][j as usize] == 0 {
                    queue.push_back((i, j, 0));
                    ans[i as usize][j as usize] = 0;
                }
            }
        }

        while let Some((i, j, d)) = queue.pop_front() {
            if i < 0 || i >= m || j < 0 || j >= n || visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));
            if ans[i as usize][j as usize] < d {
                continue;
            }

            ans[i as usize][j as usize] = d;

            queue.push_back((i + 1, j, d + 1));
            queue.push_back((i - 1, j, d + 1));
            queue.push_back((i, j + 1, d + 1));
            queue.push_back((i, j - 1, d + 1));
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
