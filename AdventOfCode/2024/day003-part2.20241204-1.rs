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

    println!("{}", Solver::new().solve(lines.join("\n").as_str()));

    Ok(())
}

pub struct Solver {
    re_mul: Regex,
    re_dont: Regex,
    re_do: Regex,
}

impl Solver {
    pub fn new() -> Solver {
        Self {
            re_mul: Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap(),
            re_dont: Regex::new(r"don't\(\)").unwrap(),
            re_do: Regex::new(r"do\(\)").unwrap(),
        }
    }

    // xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // = 48
    pub fn solve(&self, line: &str) -> i32 {
        let mut v = 0;

        for (i, s1) in self.re_dont.split(&line).enumerate() {
            if i == 0 {
                v += self.solve_line(s1);
                continue;
            }

            let ss2 = self.re_do.splitn(s1, 2).collect::<Vec<&str>>();
            if ss2.len() == 1 {
                continue;
            }

            v += self.solve_line(ss2[1]);
        }

        v
    }

    fn solve_line(&self, line: &str) -> i32 {
        let mut v = 0;
        for (_, [v1, v2]) in self.re_mul.captures_iter(line).map(|c| c.extract()) {
            let v1 = v1.parse::<i32>().unwrap();
            let v2 = v2.parse::<i32>().unwrap();
            v += v1 * v2;
        }

        v
    }
}
