use std::fmt::Debug;
use std::io::{self};

fn solve(arr: &[i32]) -> i32 {
    // 0 1 2 3 4
    // 4 2 1 5 3
    //
    // 2 1 4 0 3
    // 1 2 3 4 5

    0
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    io::stdin().read_line(&mut buf)?;
    let arr = split::<i32>(&buf, n);
    buf.clear();

    println!("{}", solve(&arr));

    Ok(())
}

pub fn split<T>(s: &str, size: usize) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    s.trim()
        .splitn(size, ' ')
        .map(|token| token.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
