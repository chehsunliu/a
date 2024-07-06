use std::collections::HashMap;
use std::io::{self};

fn solve(s: &str) -> Option<String> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in s.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }

    let mut odds = vec![];
    for (&c, &count) in &counts {
        if count % 2 == 1 {
            odds.push(c);
        }
    }

    if odds.len() > 1 {
        return None;
    }

    let mut middle = String::new();
    if let Some(c) = odds.pop() {
        let count = counts.remove(&c).unwrap();
        for _ in 0..count {
            middle.push(c);
        }
    }

    let mut half1 = String::new();
    for (c, count) in counts {
        for _ in 0..(count / 2) {
            half1.push(c);
        }
    }
    let half2 = half1.chars().rev().collect::<String>();

    let mut ans = String::new();
    ans.push_str(&half1);
    ans.push_str(&middle);
    ans.push_str(&half2);

    Some(ans)
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        match solve(buf.trim()) {
            Some(s) => println!("{s}"),
            None => println!("NO SOLUTION"),
        }
        buf.clear();
    }

    Ok(())
}
