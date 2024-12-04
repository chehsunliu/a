use std::cmp::min;
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
        let presents = lines
            .iter()
            .map(|s| {
                let vs = s
                    .split('x')
                    .map(|ss| ss.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (vs[0], vs[1], vs[2])
            })
            .collect::<Vec<(i32, i32, i32)>>();

        presents
            .iter()
            .map(|p| {
                let v1 = min(p.0 + p.1, p.1 + p.2);
                let v1 = min(v1, p.0 + p.2);
                v1 * 2 + p.0 * p.1 * p.2
            })
            .sum()
    }
}
