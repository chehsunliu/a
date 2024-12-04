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
        let depths = lines
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let windows = depths[0..depths.len() - 2]
            .iter()
            .enumerate()
            .map(|(i, _)| depths[i] + depths[i + 1] + depths[i + 2])
            .collect::<Vec<i32>>();

        let mut ans = 0;

        for i in 1..windows.len() {
            if windows[i] > windows[i - 1] {
                ans += 1;
            }
        }

        ans
    }
}
