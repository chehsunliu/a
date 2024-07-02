impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        search(&matrix, target, (matrix.len() as i32 - 1, 0))
    }
}

fn search(matrix: &Vec<Vec<i32>>, target: i32, (i, j): (i32, i32)) -> bool {
    if i < 0 || i >= matrix.len() as i32 || j < 0 || j >= matrix[0].len() as i32 {
        return false;
    }

    let current_value = matrix[i as usize][j as usize];
    if current_value == target {
        true
    } else if current_value > target {
        search(matrix, target, (i - 1, j))
    } else {
        search(matrix, target, (i, j + 1))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
