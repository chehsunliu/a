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

fn solve(a: i32, b: i32, c: i32) -> bool {
    a + b == c
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
        );

        if ans {
            writeln!(out, "+").unwrap();
        } else {
            writeln!(out, "-").unwrap();
        }
    }
}
