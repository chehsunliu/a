fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds_indexes = vec![];
    let mut odds = vec![];
    let mut ans = arr.iter().map(|v| *v).collect::<Vec<i32>>();

    for (i, v) in arr.iter().enumerate() {
        if *v % 2 == 1 {
            odds_indexes.push(i);
            odds.push(*v);
        }
    }

    odds.sort();
    for i in 0..odds.len() {
        ans[odds_indexes[i]] = odds[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
