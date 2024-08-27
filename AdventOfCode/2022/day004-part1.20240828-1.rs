use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        let (r1, r2) = buf.trim().split_once(",").unwrap();
        let r1 = parse_range(r1);
        let r2 = parse_range(r2);

        ans += if is_counted(r1, r2) || is_counted(r2, r1) {
            1
        } else {
            0
        };

        buf.clear();
    }

    println!("{}", ans);

    Ok(())
}

fn parse_range(r: &str) -> (i32, i32) {
    let (s1, s2) = r.split_once("-").unwrap();
    (s1.parse().unwrap(), s2.parse().unwrap())
}

fn is_counted(r1: (i32, i32), r2: (i32, i32)) -> bool {
    r1.0 <= r2.0 && r1.1 >= r2.1
}
