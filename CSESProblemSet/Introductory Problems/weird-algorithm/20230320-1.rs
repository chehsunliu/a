use std::io;
use std::io::{ErrorKind, Write};

fn solve(mut n: i64) -> Vec<i64> {
    let mut ans = vec![];

    while n != 1 {
        ans.push(n);
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 }
    }
    ans.push(1);

    ans
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let n = buffer
        .trim_end()
        .parse()
        .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

    let s = solve(n)
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        + "\n";

    io::stdout().write_all(s.as_bytes())?;

    Ok(())
}
