use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut records: Vec<(String, Vec<i32>)> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        records.push(
            buf.trim()
                .split_once(' ')
                .map(|(s1, s2)| {
                    let counts = s2
                        .split(',')
                        .map(|t| t.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (s1.to_string(), counts)
                })
                .unwrap(),
        );
        buf.clear();
    }

    let mut ans = 0;
    for (spring, counts) in &records {
        println!("--- {} {:?} ---", spring, counts);

        let mut partial_ans = 0;
        solve(
            &spring.chars().collect::<Vec<char>>(),
            &counts,
            &mut partial_ans,
        );

        println!("--- {} ---", partial_ans);
        ans += partial_ans;
    }

    println!("{}", ans);

    Ok(())
}

fn solve(springs: &[char], counts: &[i32], ans: &mut i32) {
    if springs.is_empty() && counts.is_empty() {
        *ans += 1;
        return;
    }

    if springs.is_empty() {
        return;
    }

    let f = || {
        let mut i: usize = 0;
        while i < springs.len() && (i as i32) < counts[0] {
            if springs[i] == '.' {
                break;
            }

            i += 1
        }

        if i as i32 != counts[0] || (i < springs.len() && springs[i] == '#') {
            None
        } else if i >= springs.len() {
            Some(i)
        } else {
            Some(i + 1)
        }
    };

    match springs[0] {
        '.' => solve(&springs[1..], counts, ans),
        '#' => {
            if counts.is_empty() {
                return;
            }

            if let Some(i) = f() {
                solve(&springs[i..], &counts[1..], ans);
            }
        }
        '?' => {
            solve(&springs[1..], counts, ans);
            if counts.is_empty() {
                return;
            }

            if let Some(i) = f() {
                solve(&springs[i..], &counts[1..], ans);
            }
        }
        _ => unreachable!(),
    }
}
