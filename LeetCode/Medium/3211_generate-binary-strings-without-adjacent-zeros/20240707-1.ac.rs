impl Solution {
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut all_s = vec![];

        for i in 0..(1 << n) {
            let s = format!("{:b}", i);
            let mut s2 = "0".repeat(n as usize - s.len());
            s2.push_str(&s);
            all_s.push(s2);
        }

        let mut ans = vec![];
        for s in all_s {
            let mut is_valid = true;
            for i in 0..(s.len() - 1) {
                let ss = &s[i..(i+2)];
                is_valid &= ss.contains('1');
            }
            if is_valid {
                ans.push(s);
            }
        }

        ans
    }
}