use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<i32>().unwrap();
        let mut values = (1..=n).collect();
        match solve(&mut values, n + 2, n) {
            Some(mut ans) => {
                ans.pop();
                println!(
                    "{}",
                    ans.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            None => println!("NO SOLUTION"),
        }
        buf.clear();
    }

    Ok(())
}

fn solve(values: &mut HashSet<i32>, previous_value: i32, n: i32) -> Option<Vec<i32>> {
    if n == 0 {
        return Some(vec![previous_value]);
    }

    let fixed_values = values.iter().map(|i| *i).collect::<Vec<i32>>();

    for value in fixed_values {
        if (value - previous_value).abs() == 1 {
            continue;
        }

        values.remove(&value);
        let result = solve(values, value, n - 1);
        values.insert(value);
        if result.is_some() {
            return result.map(|mut vs| {
                vs.push(previous_value);
                vs
            });
        }
    }

    None
}
