use std::cmp::max;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let s1 = buf.trim().to_string();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let s2 = buf.trim().to_string();
        buf.clear();

        println!("{}", solve(s1.chars().collect(), s2.chars().collect()));
    }

    Ok(())
}

fn solve(s1: Vec<char>, s2: Vec<char>) -> usize {
    let mut overlap = 0;

    for i in 0..s2.len() {
        for j in (i + 1)..(s2.len() + 1) {
            let mut k = 0;
            let mut is_matched = true;
            for c in &s2[i..j] {
                while k < s1.len() && s1[k] != *c {
                    k += 1;
                }

                if k < s1.len() {
                    k += 1;
                } else {
                    is_matched = false;
                    break;
                }
            }
            if is_matched {
                overlap = max(overlap, j - i);
            }
        }
    }
    s2.len() + s1.len() - overlap
}
