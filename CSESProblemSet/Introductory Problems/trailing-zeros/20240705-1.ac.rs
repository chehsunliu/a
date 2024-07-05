use std::io::{self};

fn solve(mut n: i64) -> i64 {
    let mut ans = 0;
    while n > 0 {
        n = n / 5;
        ans += n;
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
