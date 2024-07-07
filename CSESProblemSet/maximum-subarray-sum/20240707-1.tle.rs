use std::cmp::max;
use std::io::{self};

fn solve(nums: Vec<i32>) -> i32 {
    let mut max_sum = 0;

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
            .map(|token| token.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        buf.clear();
        println!("{}", solve(nums));
    }

    Ok(())
}
