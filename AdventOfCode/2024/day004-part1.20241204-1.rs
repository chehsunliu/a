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
        let offsets = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];

        for (i, line) in self.lines.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                if c == 'X' {
                    for &offset in &offsets {
                        v += self.traverse((i as i32, j as i32), offset, "MAS");
                    }
                }
            }
        }

        v
    }

    fn traverse(&self, pos: (i32, i32), offset: (i32, i32), pattern: &str) -> i32 {
        if pattern.is_empty() {
            return 1;
        }

        let (i, j) = (pos.0 + offset.0, pos.1 + offset.1);
        if i < 0 || j < 0 || i >= self.lines.len() as i32 || j >= self.lines[0].len() as i32 {
            return 0;
        }

        if self.lines[i as usize][j as usize] != pattern.chars().nth(0).unwrap() {
            return 0;
        }

        self.traverse((i, j), offset, &pattern[1..])
    }
}
