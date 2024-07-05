use std::io::{self};

fn solve(i: i64, j: i64) -> i64 {
    if i > j {
        if i % 2 == 0 {
            i * i - j + 1
        } else {
            (i - 1) * (i - 1) + 1 + j - 1
        }
    } else if i < j {
        if j % 2 == 0 {
            (j - 1) * (j - 1) + 1 + i - 1
        } else {
            j * j - i + 1
        }
    } else {
        i * i - i + 1
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let (s1, s2) = buf.trim().split_once(' ').unwrap();
        let (i, j) = (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap());
        println!("{}", solve(i, j));
        buf.clear();
    }

    Ok(())
}
