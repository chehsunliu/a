impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut positions = position;
        positions.sort();

        let mut l = 1;
        let mut r = positions[positions.len() - 1] - positions[0];

        while l <= r {
            let d = l + (r - l) / 2;
            if smaller_than_d_is_valid(&positions, m, d) {
                l = d + 1;
            } else {
                r = d - 1;
            }
        }

        l - 1
    }
}

fn smaller_than_d_is_valid(positions: &[i32], mut balls: i32, d: i32) -> bool {
    let mut l: i32 = 0;
    balls -= 1;

    while balls > 0 {
        let pos_base = positions[l as usize];
        l += 1;
        let mut r = positions.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let pos_m = positions[m as usize];
            if pos_m - pos_base > d {
                r = m - 1;
            } else if pos_m - pos_base < d {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        if l >= positions.len() as i32 {
            return false;
        }

        balls -= 1;
    }

    true
}

// 1 2 3 4 7

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 1), true);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 2), true);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 3), true);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 4), false);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 5), false);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 6), false);
        assert_eq!(smaller_than_d_is_valid(&vec![1, 2, 3, 4, 7], 3, 7), false);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(Solution::max_distance(vec![5, 4, 3, 2, 1, 1000], 2), 999);
    }
}
