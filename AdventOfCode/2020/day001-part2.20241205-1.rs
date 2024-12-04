use std::collections::HashMap;
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
        let values = lines
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut m: HashMap<i32, usize> = HashMap::new();
        values.iter().enumerate().for_each(|(i, &v)| {
            m.insert(v, i);
        });

        for i in 0..values.len() {
            for j in i + 1..values.len() {
                if let Some(&k) = m.get(&(2020 - values[i] - values[j])) {
                    if i != k && j != k {
                        return values[i] * values[j] * values[k];
                    }
                }
            }
        }

        panic!("GG");
    }
}
