use std::cmp::max;
use std::io::{self};

fn solve(nums: Vec<i64>) -> i64 {
    let mut max_sum = i64::MIN;

    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            max_sum = max(max_sum, sum);
        }
    }

    max_sum
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let nums = buf
            .trim()
            .split_whitespace()
            .map(|token| token.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        buf.clear();
        println!("{}", solve(nums));
    }

    Ok(())
}
