use std::collections::HashSet;
use std::io::{self};

fn solve(numbers: &Vec<i32>) -> i32 {
    numbers.iter().cloned().collect::<HashSet<i32>>().len() as i32
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut answers: Vec<i32> = vec![];

    while 0 != io::stdin().read_line(&mut buf)? {
        let n = buf.trim_end().parse::<i32>().unwrap();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let numbers = buf
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        buf.clear();

        assert_eq!(n, numbers.len() as i32);

        answers.push(solve(&numbers));
    }

    let output = answers
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{output}");

    Ok(())
}
