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
        ans += if is_safe(report) || is_really_safe(report) {
            1
        } else {
            0
        };
    }

    ans
}

fn is_safe(report: &[i32]) -> bool {
    let mut prev_v = report[0];
    let mut safe = true;
    let mut flags = (false, false);

    for &v in &report[1..] {
        let diff = v - prev_v;
        safe &= diff.abs() >= 1 && diff.abs() <= 3;
        flags = (flags.0 || (diff >= 0), flags.1 || (diff < 0));
        prev_v = v;
    }

    safe && (flags.0 ^ flags.1)
}

fn is_really_safe(report: &[i32]) -> bool {
    for (i, _) in report.iter().enumerate() {
        let mut new_report = vec![];
        for (j, &v) in report.iter().enumerate() {
            if i != j {
                new_report.push(v);
            }
        }
        if is_safe(&new_report) {
            return true;
        }
    }

    false
}
