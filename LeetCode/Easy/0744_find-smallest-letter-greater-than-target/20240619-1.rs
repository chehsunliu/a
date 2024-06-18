impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l: i32 = 0;
        let mut r: i32 = letters.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let c = letters[m as usize];
            if c > target {
                r = m - 1;
            } else if c < target {
                l = m + 1;
            } else {
                l = m + 1;
            }
        }

        if l == letters.len() as i32 {
            letters[0]
        } else {
            letters[l as usize]
        }
    }
}

struct Solution;
