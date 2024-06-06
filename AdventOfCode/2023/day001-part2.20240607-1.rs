use std::io::{self};
use std::str;

fn solve(lines: Vec<String>) -> i32 {
    let mut ans = 0;
    let letter_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in lines.iter() {
        let mut digits: Vec<i32> = vec![];
        let bytes = line.as_bytes();

        for (i, c) in bytes.iter().enumerate() {
            if *c >= b'0' && *c <= b'9' {
                digits.push((*c - b'0') as i32);
            } else if *c >= b'a' && *c <= b'z' {
                for (k, &letter_digit) in letter_digits.iter().enumerate() {
                    if i + letter_digit.len() > bytes.len() {
                        continue;
                    }

                    let s = str::from_utf8(&bytes[i..i + letter_digit.len()]).unwrap();
                    if s == letter_digit {
                        digits.push(k as i32 + 1);
                    }
                }
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
