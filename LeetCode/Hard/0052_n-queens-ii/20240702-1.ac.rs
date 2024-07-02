use std::collections::HashSet;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        f(&mut HashSet::new(), n, 0)
    }
}

fn f(queens: &mut HashSet<(i32, i32)>, n: i32, r: i32) -> i32 {
    if r >= n {
        return if queens.len() as i32 == n { 1 } else { 0 };
    }

    let mut count = 0;

    for i in 0..n {
        let pos = (r, i);
        if !is_placeable(queens, pos) {
            continue;
        }

        queens.insert(pos);
        count += f(queens, n, r + 1);
        queens.remove(&pos);
    }

    count
}

fn is_placeable(queens: &HashSet<(i32, i32)>, (i, j): (i32, i32)) -> bool {
    for &q in queens {
        if q.0 == i || q.1 == j {
            return false;
        }
        if (q.0 - i).abs() == (q.1 - j).abs() {
            return false;
        }
    }
    true
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
