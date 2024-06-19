use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut s_iter = buf.trim().splitn(2, ": ");
        s_iter.next();

        let s = s_iter.next().unwrap();
        let mut s_iter = s.trim().splitn(2, " | ");

        let s1 = s_iter.next().unwrap();
        let s2 = s_iter.next().unwrap();

        let winning_nums = s1
            .split_whitespace()
            .map(|r| r.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let nums = s2
            .split_whitespace()
            .map(|r| r.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        buf.clear();

        ans += solve(winning_nums, nums);
    }

    println!("{}", ans);

    Ok(())
}

fn solve(winning_nums: Vec<i32>, nums: Vec<i32>) -> i32 {
    let winning_nums = winning_nums.into_iter().collect::<HashSet<i32>>();
    let mut count = 0;

    for num in &nums {
        if winning_nums.contains(&num) {
            count += 1;
        }
    }

    if count > 0 {
        1 << (count - 1)
    } else {
        0
    }
}
