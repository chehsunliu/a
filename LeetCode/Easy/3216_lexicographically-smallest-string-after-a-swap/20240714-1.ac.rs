impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut ans = s.clone();
        let s = s.chars().collect::<Vec<char>>();

        for i in 0..(s.len() - 1) {
            if (s[i] as u8) % 2 == (s[i + 1] as u8) % 2 {
                let mut tmp = s.clone();
                tmp.swap(i, i + 1);

                let tmp: String = tmp.into_iter().collect();
                if tmp < ans {
                    ans = tmp;
                }
            }
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
