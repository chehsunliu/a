use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut platforms: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        platforms.push(buf.trim().chars().collect());
        buf.clear();
    }

    println!("{}", solve(platforms));

    Ok(())
}

fn solve(platforms: Vec<Vec<char>>) -> i32 {
    let m = platforms.len();
    let n = platforms[0].len();

    let mut ans = 0;

    for j in 0..n {
        let mut circles = 0;
        let mut cube_index: Option<usize> = None;

        for i in 0..m {
            match platforms[i][j] {
                'O' => {
                    circles += 1;
                }
                '#' => {
                    let base = if let Some(cube_index) = cube_index {
                        m - cube_index - 1
                    } else {
                        m
                    };
                    ans += (base + (base - circles + 1)) * circles / 2;

                    cube_index = Some(i);
                    circles = 0;
                }
                _ => {}
            }
        }

        let base = if let Some(cube_index) = cube_index {
            m - cube_index - 1
        } else {
            m
        };
        ans += (base + (base - circles + 1)) * circles / 2;
    }

    ans as i32
}
