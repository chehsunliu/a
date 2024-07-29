impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;

        for j in 0..rating.len() {
            let mut count_00 = 0;
            let mut count_10 = 0;
            for i in 0..j {
                if rating[i] < rating[j] {
                    count_00 += 1;
                } else if rating[i] > rating[j] {
                    count_10 += 1;
                }
            }

            let mut count_01 = 0;
            let mut count_11 = 0;
            for k in (j + 1)..rating.len() {
                if rating[j] < rating[k] {
                    count_01 += 1;
                } else if rating[j] > rating[k] {
                    count_11 += 1;
                }
            }

            count += count_00 * count_01 + count_10 * count_11;
        }

        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
