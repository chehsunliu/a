use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let m = image.len() as i32;
        let n = image[0].len() as i32;

        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        stack.push_back((sr, sc));
        visited.insert((sr, sc));
        let current_color = image[sr as usize][sc as usize];

        while let Some((i, j)) = stack.pop_back() {
            if i < 0 || i >= m || j < 0 || j >= n {
                continue;
            }

            if image[i as usize][j as usize] != current_color {
                continue;
            }

            image[i as usize][j as usize] = color;

            let neighbors = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    stack.push_back(neighbor);
                }
            }
        }

        image
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
