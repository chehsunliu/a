use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        let s = buf.trim();
        let s1 = &s[..(s.len() / 2)];
        let s2 = &s[(s.len() / 2)..];

        let s1: HashSet<char> = HashSet::from_iter(s1.chars());
        let s2 = HashSet::from_iter(s2.chars());
        let s_common = s1.intersection(&s2);

        for &c in s_common {
            ans += match c {
                'a'..='z' => c as i32 - 'a' as i32 + 1,
                'A'..='Z' => c as i32 - 'A' as i32 + 27,
                _ => unreachable!(),
            }
        }

        buf.clear();
    }

    println!("{}", ans);

    Ok(())
}
