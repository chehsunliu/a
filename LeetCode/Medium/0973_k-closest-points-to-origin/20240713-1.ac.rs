impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points
            .into_iter()
            .map(|p| {
                let d = p[0].pow(2) + p[1].pow(2);
                (d, p)
            })
            .collect::<Vec<(i32, Vec<i32>)>>();
        points.sort();

        Vec::from(&points[0..k as usize])
            .into_iter()
            .map(|(_, p)| p)
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
