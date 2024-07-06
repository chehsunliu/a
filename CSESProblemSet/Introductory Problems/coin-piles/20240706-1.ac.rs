use std::io::{self};

fn solve(mut a: i64, mut b: i64) -> bool {
    if a > b {
        (a, b) = (b, a);
    }

    if (a + b) % 3 != 0 {
        return false;
    }

    if 2 * a < b {
        return false;
    }

    true
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let (s1, s2) = buf.trim().split_once(' ').unwrap();
        let (a, b) = (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap());
        buf.clear();

        if solve(a, b) {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    Ok(())
}
