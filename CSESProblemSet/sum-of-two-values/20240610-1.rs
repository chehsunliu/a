use std::fmt::Debug;
use std::io::{self};

fn solve(aa: Vec<i32>, x: i32) -> Option<(usize, usize)> {
    let mut aa: Vec<(usize, i32)> = aa.into_iter().enumerate().collect();
    aa.sort_by_key(|a| a.1);

    let mut l = 0;
    let mut r = aa.len() - 1;
    let mut ans = None;

    while l < r {
        let v = aa[r].1 + aa[l].1;

        if v == x {
            ans = Some((aa[r].0 + 1, aa[l].0 + 1));
            break;
        } else if v > x {
            r -= 1;
        } else {
            l += 1;
        }
    }

    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut buf_iter = buf.trim().splitn(2, ' ');
        let n: usize = buf_iter.next().unwrap().parse().unwrap();
        let x: i32 = buf_iter.next().unwrap().parse().unwrap();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let aa = split::<i32>(&buf, n);
        buf.clear();

        match solve(aa, x) {
            Some((i, j)) => println!("{} {}", i, j),
            None => println!("IMPOSSIBLE"),
        }
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
