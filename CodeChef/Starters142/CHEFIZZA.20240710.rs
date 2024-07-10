use std::io::{self};

fn solve(x: i32) -> i32 {
    let mut v = 2;
    while v < x {
        v *= 2;
    }
    if v == x {
        0
    } else {
        (x - v / 2) / 2 * 4
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let x = buf.trim().parse::<i32>().unwrap();
        buf.clear();
        println!("{}", solve(x));
    }

    Ok(())
}
