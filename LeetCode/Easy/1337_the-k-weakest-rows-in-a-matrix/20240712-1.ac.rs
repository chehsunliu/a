impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rows = mat
            .iter()
            .enumerate()
            .map(|(i, row)| (row.iter().sum::<i32>(), i))
            .collect::<Vec<(i32, usize)>>();

        rows.sort();

        Vec::from(&rows[..k as usize])
            .iter()
            .map(|&r| r.1 as i32)
            .collect::<Vec<i32>>()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
