use std::collections::HashMap;
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
    let mut tokens: Vec<(usize, usize, usize)> = vec![];

    for i in 0..lines.len() {
        let mut token: Option<(usize, usize)> = None;

        for j in 0..lines[0].len() {
            let c = lines[i][j];
            if c.is_digit(10) {
                token = Some(token.map_or((j, 1), |(fixed_j, len)| (fixed_j, len + 1)));
            } else {
                if let Some((fixed_j, len)) = token {
                    tokens.push((i, fixed_j, len));
                }
                token = None;
            }
        }

        if let Some((fixed_j, len)) = token {
            tokens.push((i, fixed_j, len));
        }
    }

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for &(i_base, j_base, len) in &tokens {
        let v = String::from_iter(&lines[i_base][j_base..j_base + len])
            .parse::<i32>()
            .unwrap();

        for i_offset in -1..2 {
            for j_offset in -1..len as i32 + 1 {
                let i = i_base as i32 + i_offset;
                let j = j_base as i32 + j_offset;

                if i < 0 || i >= lines.len() as i32 || j < 0 || j >= lines[0].len() as i32 {
                    continue;
                }

                let c = lines[i as usize][j as usize];
                if c == '*' {
                    let k = (i as usize, j as usize);
                    if gears.contains_key(&k) {
                        gears.get_mut(&k).unwrap().push(v);
                    } else {
                        gears.insert(k, vec![v]);
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for (_, values) in gears {
        if values.len() == 2 {
            ans += values.iter().fold(1, |acc, x| x * acc);
        }
    }

    ans
}
