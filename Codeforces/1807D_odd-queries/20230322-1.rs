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

fn main() {
    let mut scanner = Scanner::new(io::stdin().lock());
    let mut out = BufWriter::new(io::stdout());

    for _ in 0..scanner.scan::<i32>() {
        let n = scanner.scan::<usize>();
        let q = scanner.scan::<i32>();

        let mut odds_cum = vec![0; n];
        let mut odds = 0;

        for i in 0..n {
            let a = scanner.scan::<i32>();
            odds += if a % 2 == 1 { 1 } else { 0 };
            odds_cum[i] = odds;
        }

        for _ in 0..q {
            let l = scanner.scan::<usize>();
            let r = scanner.scan::<usize>();
            let k = scanner.scan::<i32>();

            let local_odds = odds_cum[r - 1] - (if l > 1 { odds_cum[l - 2] } else { 0 });
            let new_odds = odds - local_odds + (k % 2) * ((r - l + 1) as i32);

            if new_odds % 2 == 1 {
                writeln!(out, "YES").unwrap();
            } else {
                writeln!(out, "NO").unwrap();
            }
        }
    }
}
