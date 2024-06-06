use std::io::{self};

fn solve(lines: Vec<String>) -> i32 {
    let mut ans = 0;

    for line in lines.iter() {
        let mut digits: Vec<i32> = vec![];

        for c in line.as_bytes() {
            if *c >= b'0' && *c <= b'9' {
                digits.push((*c - b'0') as i32);
            }
        }

        if digits.len() > 0 {
            ans += digits[0] * 10 + digits[digits.len() - 1];
        }
    }

    ans
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut lines: Vec<String> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        lines.push(buf.trim_end().to_string());
        buf.clear();
    }

    println!("{}", solve(lines));

    Ok(())
}
