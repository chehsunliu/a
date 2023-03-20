use std::io;
use std::io::{BufRead, BufWriter, Write};

struct Scanner<T: BufRead> {
    buffer: Vec<String>,
    reader: T,
}

impl<T: BufRead> Scanner<T> {
    pub fn new(reader: T) -> Scanner<T> {
        Scanner {
            buffer: vec![],
            reader,
        }
    }

    pub fn scan<R>(&mut self) -> R
    where
        R: std::str::FromStr,
        <R as std::str::FromStr>::Err: std::fmt::Debug,
    {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().unwrap();
            }

            let mut s = String::new();
            self.reader.read_line(&mut s).unwrap();
            self.buffer = s
                .split_whitespace()
                .map(String::from)
                .rev()
                .collect::<Vec<String>>()
        }
    }
}

fn solve(a: i32, b: i32, c: i32, d: i32) -> i32 {
    if d < b {
        return -1;
    }

    if c > a + (d - b) {
        return -1;
    }

    (d - b) + (a + (d - b) - c)
}

fn main() {
    let mut scanner = Scanner::new(io::stdin().lock());
    let mut out = BufWriter::new(io::stdout());

    let t = scanner.scan::<i32>();
    for _ in 0..t {
        let ans = solve(
            scanner.scan::<i32>(),
            scanner.scan::<i32>(),
            scanner.scan::<i32>(),
            scanner.scan::<i32>(),
        );
        writeln!(out, "{}", ans.to_string()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve(-1, 0, -1, 2), 4);
        assert_eq!(solve(0, 0, 4, 5), 6);
        assert_eq!(solve(-2, -1, 1, 1), -1);
        assert_eq!(solve(-3, 2, -3, 2), 0);
        assert_eq!(solve(2, -1, -1, -1), 3);
        assert_eq!(solve(1, 1, 0, 2), 3);
    }
}
