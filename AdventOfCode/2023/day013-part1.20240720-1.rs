use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut patterns: Vec<Vec<Vec<char>>> = vec![];
    let mut pattern: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let line = buf.trim();
        if line.is_empty() {
            assert!(!pattern.is_empty());
            patterns.push(pattern);
            pattern = vec![];
        } else {
            pattern.push(line.chars().collect());
        }
        buf.clear();
    }

    assert!(!pattern.is_empty());
    patterns.push(pattern);

    let mut ans = 0;

    for pattern in patterns {
        ans += solve(pattern);
    }

    println!("{ans}");

    Ok(())
}

fn solve(pattern: Vec<Vec<char>>) -> i32 {
    let mut reflection_h: Option<usize> = None;
    for i in 1..pattern.len() {
        let (l, r) = (i - 1, i);
        let mut offset = 0;

        while l >= offset && r + offset < pattern.len() {
            let (s1, s2) = (
                pattern[l - offset].iter().collect::<String>(),
                pattern[r + offset].iter().collect::<String>(),
            );

            if s1 != s2 {
                break;
            }

            offset += 1;
        }

        if offset > 0 {
            if i == offset || i + offset == pattern.len() {
                reflection_h = Some(i);
            }
        }
    }

    let mut reflection_v: Option<usize> = None;
    for j in 1..pattern[0].len() {
        let (l, r) = (j - 1, j);
        let mut offset = 0;

        while l >= offset && r + offset < pattern[0].len() {
            let (s1, s2) = (
                (0..pattern.len())
                    .map(|i| pattern[i][l - offset])
                    .collect::<String>(),
                (0..pattern.len())
                    .map(|i| pattern[i][r + offset])
                    .collect::<String>(),
            );

            if s1 != s2 {
                break;
            }

            offset += 1;
        }

        if offset > 0 {
            if j == offset || j + offset == pattern[0].len() {
                reflection_v = Some(j);
            }
        }
    }

    if let Some(i) = reflection_h {
        return i as i32 * 100;
    }

    if let Some(j) = reflection_v {
        return j as i32;
    }

    unreachable!();
}
