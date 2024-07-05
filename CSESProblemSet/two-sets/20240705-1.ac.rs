use std::collections::HashSet;
use std::io::{self};

fn solve(n: i64) -> Option<(Vec<i64>, Vec<i64>)> {
    let mut v1 = vec![];
    let sum = n * (n + 1) / 2;
    if sum % 2 == 0 && f(sum / 2, n, &mut v1) {
        let mut v2 = vec![];
        let v1_lookup = v1.iter().map(|v| *v).collect::<HashSet<i64>>();
        for v in 1..=n {
            if !v1_lookup.contains(&v) {
                v2.push(v);
            }
        }

        Some((v1, v2))
    } else {
        None
    }
}

fn f(remaining_sum: i64, n: i64, ans: &mut Vec<i64>) -> bool {
    if n == 0 {
        return if remaining_sum == 0 { true } else { false };
    }

    if remaining_sum == 0 {
        return true;
    }

    if n > remaining_sum {
        ans.push(remaining_sum);
        return true;
    }

    ans.push(n);
    if f(remaining_sum - n, n - 1, ans) {
        return true;
    }
    ans.pop();
    return f(remaining_sum, n - 1, ans);
}

fn vec_to_string(v: Vec<i64>) -> String {
    v.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<i64>().unwrap();
        buf.clear();

        match solve(n) {
            Some((vec1, vec2)) => {
                println!("YES");
                println!("{}", vec1.len());
                println!("{}", vec_to_string(vec1));
                println!("{}", vec2.len());
                println!("{}", vec_to_string(vec2));
            }
            None => {
                println!("NO");
            }
        }
    }

    Ok(())
}

// 1 -> 1
// 2 -> 3
// 3 -> 6
// 4 -> 10 | 1 2 3 4
// 5 -> 15
// 6 -> 21
// 7 -> 28 | 1 7 6 3 4 5 2
// 8 -> 36 | 1 2 3 4 5 6 7 8
// 9 -> 45
