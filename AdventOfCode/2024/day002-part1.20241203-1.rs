use std::collections::HashMap;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut reports: Vec<Vec<i32>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let report: Vec<i32> = buf
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
        buf.clear();
    }

    println!("{}", solve(reports));

    Ok(())
}

fn solve(reports: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for report in &reports {
        let &(_, r, c) = &report[1..].iter().fold(
            (report[0], true, (0, 0)),
            |(prev_v, prev_r, (mut c1, mut c2)), &v| {
                let diff = (prev_v - v).abs();
                if prev_v > v {
                    c1 += 1;
                } else {
                    c2 += 1;
                }
                (v, prev_r && (diff >= 1 && diff <= 3), (c1, c2))
            },
        );
        ans += if r && ((c.0 == 0 && c.1 != 0) || (c.0 != 0 && c.1 == 0)) {
            1
        } else {
            0
        };
    }

    ans
}
