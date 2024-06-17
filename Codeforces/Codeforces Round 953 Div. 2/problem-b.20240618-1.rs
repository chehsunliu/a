use std::cmp::{max, min};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let t: usize = buf.trim().parse().unwrap();
    buf.clear();

    for _ in 0..t {
        io::stdin().read_line(&mut buf)?;
        let mut buf_iter = buf.trim().splitn(3, ' ');

        let n = buf_iter.next().unwrap().parse::<i64>().unwrap();
        let a = buf_iter.next().unwrap().parse::<i64>().unwrap();
        let b = buf_iter.next().unwrap().parse::<i64>().unwrap();
        buf.clear();

        // b - a + 1 >= i
        let k = max(b - a + 1, 0);
        let k = min(k, n);
        let ans = (b + 1) * k - (1 + k) * k / 2 + a * (n - k);
        println!("{}", ans);
    }

    Ok(())
}
