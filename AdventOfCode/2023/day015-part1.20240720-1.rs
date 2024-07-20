use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    println!("{}", solve(buf.trim()));

    Ok(())
}

fn solve(raw: &str) -> i32 {
    let mut ans = 0;

    for s in raw.split(',') {
        let mut current_value = 0;
        for c in s.chars() {
            current_value += c as i32;
            current_value *= 17;
            current_value %= 256;
        }
        ans += current_value;
    }

    ans
}
