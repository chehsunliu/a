use regex::Regex;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut lines: Vec<String> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        lines.push(buf.trim().to_string());
        buf.clear();
    }

    println!("{}", solve(lines));

    Ok(())
}

fn solve(lines: Vec<String>) -> i32 {
    let mut ans = 0;

    for line in lines {
        ans += solve_line(line);
    }

    ans
}

fn solve_line(line: String) -> i32 {
    let re: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut v = 0;
    for (_, [v1, v2]) in re.captures_iter(line.as_str()).map(|c| c.extract()) {
        let v1 = v1.parse::<i32>().unwrap();
        let v2 = v2.parse::<i32>().unwrap();
        v += v1 * v2;
    }

    v
}
