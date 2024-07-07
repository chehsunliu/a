use std::cmp::max;
use std::io::{self};

fn solve(mut nums: Vec<i64>) -> i64 {
    let mut max_sum = *nums.last().unwrap();

    for i in (0..(nums.len() - 1)).rev() {
        nums[i] = max(nums[i], nums[i + 1] + nums[i]);
        max_sum = max(max_sum, nums[i]);
    }

    max_sum
}

// -1  3 -2  5  3 -5  2  2
//  8  9  6  8  3 -1  4  2

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
