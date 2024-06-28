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
        let mut length = 0;

        while h[length] != h[h.len() - 1] || h[h.len() - 1] != 0 {
            for i in ((length + 1)..h.len()).rev() {
                h[i] = h[i] - h[i - 1]
            }

            length += 1;
        }

        let mut temp = 0;
        for i in (0..length).rev() {
            temp = h[i] - temp;
        }
        ans += temp;
    }

    ans
}
