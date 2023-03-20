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

fn solve(values: Vec<i32>) -> bool {
    let mut counts = [0; 2];

    for value in values.iter() {
        counts[(*value % 2) as usize] += value;
    }

    counts[0] > counts[1]
}

fn main() {
    let mut scanner = Scanner::new(io::stdin().lock());
    let mut out = BufWriter::new(io::stdout());

    for _ in 0..scanner.scan::<i32>() {
        let mut values = vec![];
        for _ in 0..scanner.scan::<i32>() {
            values.push(scanner.scan::<i32>());
        }

        if solve(values) {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
