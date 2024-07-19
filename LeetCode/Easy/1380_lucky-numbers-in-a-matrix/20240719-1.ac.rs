use std::collections::HashSet;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mins: HashSet<(usize, usize)> = HashSet::new();
        let mut maxs: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..matrix.len() {
            let mut index = (i, 0);
            for j in 0..matrix[0].len() {
                if matrix[i][j] < matrix[index.0][index.1] {
                    index = (i, j);
                }
            }
            mins.insert(index);
        }

        for j in 0..matrix[0].len() {
            let mut index = (0, j);
            for i in 0..matrix.len() {
                if matrix[i][j] > matrix[index.0][index.1] {
                    index = (i, j);
                }
            }
            maxs.insert(index);
        }

        mins.intersection(&maxs)
            .collect::<HashSet<_>>()
            .iter()
            .map(|&&index| matrix[index.0][index.1])
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
