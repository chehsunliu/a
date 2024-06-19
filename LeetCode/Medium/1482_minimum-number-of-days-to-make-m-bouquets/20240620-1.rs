use std::cmp::max;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let m = m as i64;
        let k = k as i64;

        if m * k > bloom_day.len() as i64 {
            return -1;
        }

        let max_bloom_day = find_max_bloom_day(&bloom_day);

        let mut l = 1;
        let mut r = max_bloom_day;

        while l <= r {
            let current_day = l + (r - l) / 2;
            let bouquets = count_bouquets(&bloom_day, k, current_day);

            if bouquets > m {
                r = current_day - 1;
            } else if bouquets < m {
                l = current_day + 1;
            } else {
                r = current_day - 1;
            }
        }

        l as i32
    }
}

fn find_max_bloom_day(days: &[i32]) -> i64 {
    days.iter().fold(0, |max_d, d| max(max_d, *d)) as i64
}

fn count_bouquets(bloom_days: &[i32], k: i64, current_day: i64) -> i64 {
    let mut bouquets = 0;
    let mut day_count = 0;

    for bloom_day in bloom_days {
        if *bloom_day as i64 <= current_day {
            day_count += 1;
            if day_count == k {
                bouquets += 1;
                day_count = 0;
            }
        } else {
            day_count = 0;
        }
    }

    bouquets
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn search_r_works() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    }
}
