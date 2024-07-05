use std::io::{self};

fn solve(n: i64) -> i64 {
    let mut ans = 1;
    for _ in 0..n {
        ans = (ans * 2) % 1_000_000_007;
    }
    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<i64>().unwrap();
        buf.clear();

        println!("{}", solve(n))
    }

    Ok(())
}

// 1 -> 1
// 2 -> 3
// 3 -> 6
// 4 -> 10 | 1 2 3 4
// 5 -> 15
// 6 -> 21
// 7 -> 28 | 1 7 6 3 4 5 2
// 8 -> 36 | 1 2 3 4 5 6 7 8
// 9 -> 45
