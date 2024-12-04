use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut lines: Vec<String> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        lines.push(buf.trim().to_string());
        buf.clear();
    }

    println!("{}", Solver::new(lines).solve());

    Ok(())
}

pub struct Solver {
    lines: Vec<Vec<char>>,
}

impl Solver {
    pub fn new(lines: Vec<String>) -> Solver {
        Self {
            lines: lines.iter().map(|l| l.chars().collect()).collect(),
        }
    }

    pub fn solve(&self) -> i32 {
        let mut v = 0;

        for (i, line) in self.lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c != 'A' || i == 0 || i == self.lines.len() - 1 || j == 0 || j == line.len() - 1
                {
                    continue;
                }

                let d1 = self.check((i - 1, j - 1), (i + 1, j + 1));
                let d2 = self.check((i + 1, j - 1), (i - 1, j + 1));
                v += if d1 && d2 { 1 } else { 0 };
            }
        }

        v
    }

    fn check(&self, p1: (usize, usize), p2: (usize, usize)) -> bool {
        let mut s = String::new();
        s.push(self.lines[p1.0][p1.1]);
        s.push(self.lines[p2.0][p2.1]);

        s == "MS" || s == "SM"
    }
}
