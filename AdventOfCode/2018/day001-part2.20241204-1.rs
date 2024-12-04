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
        let mut freq = 0;

        let mut m: HashSet<i32> = HashSet::new();
        m.insert(freq);

        loop {
            for line in &lines {
                let v: i32 = line.parse().unwrap();
                freq += v;
                if m.contains(&freq) {
                    return freq;
                } else {
                    m.insert(freq);
                }
            }
        }
    }
}
