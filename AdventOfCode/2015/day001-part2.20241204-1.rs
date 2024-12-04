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

    pub fn solve(&self, s: String) -> usize {
        let mut v = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                v += 1;
            } else {
                v -= 1;
            }

            if v < 0 {
                return i + 1;
            }
        }

        0
    }
}
