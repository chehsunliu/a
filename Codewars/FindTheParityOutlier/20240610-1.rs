fn find_outlier(values: &[i32]) -> i32 {
    let mut counts = (0, 0);

    for i in 0..3 {
        match values[i] % 2 {
            0 => counts.0 += 1,
            _ => counts.1 += 1,
        }
    }

    let target = if counts.0 > counts.1 { 1 } else { 0 };
    for value in values {
        if value % 2 == target || value % 2 == -target {
            return *value;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
