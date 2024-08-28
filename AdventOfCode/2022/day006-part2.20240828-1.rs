use std::collections::{HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let distinct_count = 14;

    while io::stdin().read_line(&mut buf)? != 0 {
        let cs = buf.trim().chars().collect::<Vec<char>>();
        for i in (distinct_count - 1)..cs.len() {
            let s: HashSet<&char> = HashSet::from_iter(&cs[(i - (distinct_count - 1))..=i]);
            if s.len() == distinct_count {
                println!("{}", i + 1);
                break;
            }
        }

        buf.clear();
    }

    Ok(())
}
