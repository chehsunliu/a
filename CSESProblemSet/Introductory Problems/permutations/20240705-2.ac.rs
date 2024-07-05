use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<i32>().unwrap();
        println!("{}", solve(n));
        buf.clear();
    }

    Ok(())
}

fn solve(n: i32) -> String {
    if n <= 3 && n != 1 {
        return "NO SOLUTION".to_string();
    }

    let mut ans: Vec<i32> = vec![];
    for v in 1..=n {
        if v % 2 == 0 {
            ans.push(v);
        }
    }
    for v in 1..=n {
        if v % 2 == 1 {
            ans.push(v);
        }
    }

    ans.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// 2 4 1 3
// 2 4 1 3 5
// 2 4 6 1 3 5
// 1 3 5 2 4
// 1 3 5 2 4 6
// 1 3 5 7 2 4 6
