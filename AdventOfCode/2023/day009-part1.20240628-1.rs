use std::fmt::Debug;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut histories: Vec<Vec<i64>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let history = buf
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        histories.push(history);
        buf.clear();
    }

    println!("{}", solve(histories));

    Ok(())
}

fn solve(mut histories: Vec<Vec<i64>>) -> i64 {
    let mut ans: i64 = 0;

    for h in &mut histories {
        let mut length = h.len();

        while h[0] != h[length - 1] || h[0] != 0 {
            for i in 0..(length - 1) {
                h[i] = h[i + 1] - h[i]
            }

            length -= 1;
        }

        ans += h[(length - 1)..h.len()].iter().sum::<i64>();
    }

    ans
}
