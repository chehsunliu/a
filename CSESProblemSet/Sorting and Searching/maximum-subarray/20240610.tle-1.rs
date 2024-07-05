use std::cmp::max;
use std::fmt::Debug;
use std::io::{self};

fn solve(xs: Vec<i64>) -> i64 {
    let mut sums: Vec<i64> = vec![0; xs.len() + 1];

    for i in (0..xs.len()).rev() {
        sums[i] = xs[i] + sums[i + 1];
    }

    let mut ans = xs[0];

    for i in 0..(sums.len() - 1) {
        for j in (i + 1)..sums.len() {
            ans = max(ans, sums[i] - sums[j]);
        }
    }

    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n: usize = buf.trim().parse().unwrap();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let xs = split::<i64>(&buf, n);
        buf.clear();

        println!("{}", solve(xs));
    }

    Ok(())
}

fn split<T>(s: &str, size: usize) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    s.trim()
        .splitn(size, ' ')
        .map(|token| token.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
