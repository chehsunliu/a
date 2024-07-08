impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut is_out = vec![false; n as usize];
        let mut i = 0;

        for _ in 0..(n - 1) {
            let mut kk = 0;
            while kk < k - 1 {
                i = (i + 1) % n;
                if is_out[i as usize] {
                    continue;
                }
                kk += 1;
            }

            is_out[i as usize] = true;
            i = (i + 1) % n;
            while is_out[i as usize] {
                i = (i + 1) % n;
            }
        }

        1 + is_out
            .iter()
            .enumerate()
            .filter(|&(_, &out)| !out)
            .last()
            .unwrap()
            .0 as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
