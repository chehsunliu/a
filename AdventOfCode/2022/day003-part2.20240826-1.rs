use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut elves: Vec<String> = vec![];

    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        elves.push(buf.trim().to_string());
        buf.clear();

        if elves.len() == 3 {
            let s1: HashSet<char> = HashSet::from_iter(elves[0].chars());
            let s2: HashSet<char> = HashSet::from_iter(elves[1].chars());
            let s3: HashSet<char> = HashSet::from_iter(elves[2].chars());

            let s: HashSet<char> = s1
                .iter()
                .filter(|&c| s2.contains(c))
                .filter(|&c| s3.contains(c))
                .map(|c| *c)
                .collect();

            for c in s {
                ans += match c {
                    'a'..='z' => c as i32 - 'a' as i32 + 1,
                    'A'..='Z' => c as i32 - 'A' as i32 + 27,
                    _ => unreachable!(),
                }
            }
            elves.clear();
        }
    }

    println!("{}", ans);

    Ok(())
}
