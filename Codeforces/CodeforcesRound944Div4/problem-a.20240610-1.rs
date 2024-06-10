use std::cmp::{max, min};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut buf_iter = buf.trim().splitn(2, ' ').into_iter();
        let x: u32 = buf_iter.next().unwrap().parse().unwrap();
        let y: u32 = buf_iter.next().unwrap().parse().unwrap();
        buf.clear();

        println!("{} {}", min(x, y), max(x, y));
    }

    Ok(())
}
