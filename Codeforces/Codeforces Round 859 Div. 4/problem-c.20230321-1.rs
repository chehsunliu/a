use std::collections::HashMap;
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

fn solve(s: String) -> bool {
    let mut cache: HashMap<u8, [i32; 2]> = HashMap::new();

    for (i, c) in s.as_bytes().iter().enumerate() {
        if !cache.contains_key(c) {
            cache.insert(*c, [0, 0]);
        }

        cache.get_mut(c).unwrap()[i % 2] += 1;
    }

    let mut ans = true;

    for counts in cache.values() {
        ans &= counts[0] == 0 || counts[1] == 0;
    }

    ans
}

fn main() {
    let mut scanner = Scanner::new(io::stdin().lock());
    let mut out = BufWriter::new(io::stdout());

    for _ in 0..scanner.scan::<i32>() {
        scanner.scan::<i32>();

        if solve(scanner.scan::<String>()) {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
