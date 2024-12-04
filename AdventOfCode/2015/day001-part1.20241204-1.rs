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

pub struct Solver {}

impl Solver {
    pub fn new() -> Solver {
        Self {}
    }

    pub fn solve(&self, s: String) -> i32 {
        let mut count1 = 0;
        let mut count2 = 0;

        for c in s.chars() {
            if c == '(' {
                count1 += 1;
            } else {
                count2 += 1;
            }
        }

        count1 - count2
    }
}
