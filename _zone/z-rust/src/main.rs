use std::io;
use std::io::Write;

fn solve(a: i32, b: i32, c: i32, d: i32) -> i32 {
    if d < b {
        return -1;
    }

    if c > a + (d - b) {
        return -1;
    }

    (d - b) + (a + (d - b) - c)
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut iter = buf.trim_end().split_whitespace();
        let ans = solve(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        io::stdout().write_all((ans.to_string() + "\n").as_bytes())?;
        buf.clear();
    }

    Ok(())
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
