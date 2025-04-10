use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut lines: Vec<String> = Vec::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        lines.push(buf.trim().to_string());
        buf.clear();
    }

    println!("{}", Solver::new().solve(lines));

    Ok(())
}

#[derive(Debug)]
pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Self {}
    }

    pub fn solve(&mut self, lines: Vec<String>) -> i32 {
        let mut ss: HashSet<i32> = HashSet::new();

        for line in &lines {
            let v: i32 = line.parse().unwrap();

            if let Some(&v0) = ss.get(&(2020 - v)) {
                return v * v0;
            } else {
                ss.insert(v);
            }
        }

        panic!("GG");
    }
}
