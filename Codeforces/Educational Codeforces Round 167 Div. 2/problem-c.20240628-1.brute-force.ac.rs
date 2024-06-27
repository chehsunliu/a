use std::cmp::{max, min};
use std::fmt::Debug;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<usize>().unwrap();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let ratings_1: Vec<i32> = split(&buf, n);
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let ratings_2: Vec<i32> = split(&buf, n);
        buf.clear();

        println!("{}", solve(ratings_1, ratings_2, n));
    }

    Ok(())
}

fn solve(ratings_1: Vec<i32>, ratings_2: Vec<i32>, n: usize) -> i32 {
    let mut rs1 = 0;
    let mut rs2 = 0;
    let mut count_plus = 0;
    let mut count_minus = 0;

    for i in 0..n {
        let r1 = ratings_1[i];
        let r2 = ratings_2[i];

        if r1 > r2 {
            rs1 += r1;
        } else if r1 < r2 {
            rs2 += r2;
        } else {
            if r1 == 1 {
                count_plus += 1;
            } else if r1 == -1 {
                count_minus += 1;
            }
        }
    }

    while count_plus > 0 {
        if rs1 > rs2 {
            rs2 += 1;
        } else {
            rs1 += 1;
        }
        count_plus -= 1;
    }

    while count_minus > 0 {
        if rs1 > rs2 {
            rs1 -= 1;
        } else {
            rs2 -= 1;
        }
        count_minus -= 1;
    }

    min(rs1, rs2)
}

pub fn split<T>(s: &str, size: usize) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    s.trim()
        .splitn(size, ' ')
        .map(|token| token.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

// -1 -1 -1  1
// -1  1  1  1

//           1
// -1  1  1

// -1        1
//     1  1

// -1 1 0 -1 -1 1
// -1 1 0  0  1 0

// 1 1 1 -> 2
// 1 1 1    1
// 1 1 1 1 -> 2 -> 4 3 2 1 0
// 1 1 1 1 -> 2 -> 0 1 2 3 4

// -1 -1 -1 -> -2 -> -3 -2 -1 0
// -1 -1 -1 -> -1 ->  0 -1 -2 -3

// 4 -3
// 5
// 1

// 1
// 3 -3
// -2 2 0
// -1 2 1
