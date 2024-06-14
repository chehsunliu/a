impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let m = s.len() / 2;

        while l < m {
            let r = s.len() - 1 - l;
            s.swap(l, r);

            l += 1;
        }
    }
}

struct Solution;
