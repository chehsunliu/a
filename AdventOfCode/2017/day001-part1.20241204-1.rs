use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        println!("{}", Solver::new().solve(buf.trim().to_string()));
        buf.clear();
    }

    Ok(())
}

#[derive(Debug)]
pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Self {}
    }

    pub fn solve(&mut self, s: String) -> u32 {
        let digits = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let mut ans = 0;

        for i in 0..digits.len() {
            let d1 = digits[i];
            let d2 = digits[(i + 1) % digits.len()];
            if d1 == d2 {
                ans += d1;
            }
        }

        ans
    }
}
