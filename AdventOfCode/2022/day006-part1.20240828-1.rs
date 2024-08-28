use std::collections::{HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let cs = buf.trim().chars().collect::<Vec<char>>();
        for i in 3..cs.len() {
            let s: HashSet<&char> = HashSet::from_iter(&cs[(i - 3)..=i]);
            if s.len() == 4 {
                println!("{}", i + 1);
                break;
            }
        }

        buf.clear();
    }

    Ok(())
}
