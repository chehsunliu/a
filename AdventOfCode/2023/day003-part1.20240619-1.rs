use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut lines: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let line = buf.trim().chars().collect::<Vec<char>>();
        lines.push(line);
        buf.clear();
    }

    println!("{}", solve(lines));

    Ok(())
}

fn solve(lines: Vec<Vec<char>>) -> i32 {
    let mut marks: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..lines.len() as i32 {
        for j in 0..lines[0].len() as i32 {
            if !is_symbol(lines[i as usize][j as usize]) {
                continue;
            }

            marks.insert((i - 1, j - 1));
            marks.insert((i - 1, j));
            marks.insert((i - 1, j + 1));
            marks.insert((i, j - 1));
            marks.insert((i, j + 1));
            marks.insert((i + 1, j - 1));
            marks.insert((i + 1, j));
            marks.insert((i + 1, j + 1));
        }
    }

    let mut ans = 0;

    for i in 0..lines.len() as i32 {
        let mut builder: Option<i32> = None;
        let mut is_valid = false;

        for j in 0..lines[0].len() as i32 {
            let c = lines[i as usize][j as usize];
            if c.is_digit(10) {
                let cv = (c as u8 - '0' as u8) as i32;
                is_valid |= marks.contains(&(i, j));
                builder = Some(builder.map_or(cv, |v| v * 10 + cv));
            } else {
                if is_valid {
                    ans += builder.unwrap_or(0);
                }
                is_valid = false;
                builder = None;
            }
        }

        if is_valid {
            ans += builder.unwrap_or(0);
        }
    }

    ans
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}
