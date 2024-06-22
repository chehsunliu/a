use std::cmp::min;
use std::io::{self};
use std::iter::zip;

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let times = buf
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect::<Vec<f64>>();
    buf.clear();

    io::stdin().read_line(&mut buf)?;
    let distances = buf
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect::<Vec<f64>>();
    buf.clear();

    let mut ans = 1;

    for (t, d) in zip(&times, &distances) {
        let term2 = (t * t - 4.0 * d).sqrt();
        let s1 = (t - term2) / 2.0;
        let s2 = (t + term2) / 2.0;

        let s1 = if s1 as i32 as f64 == s1 { s1 + 1.0 } else { s1 };
        let s2 = if s2 as i32 as f64 == s2 { s2 - 1.0 } else { s2 };

        ans *= (s2.floor() - s1.ceil() + 1.0) as i32;
    }

    println!("{ans}");

    Ok(())
}
