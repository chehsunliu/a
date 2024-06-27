use std::cmp::max;
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

    // Make sure rs1 >= rs2
    if rs1 < rs2 {
        (rs1, rs2) = (rs2, rs1);
    }
    let diff = rs1 - rs2;
    if diff > 0 {
        if diff > count_plus {
            rs2 += count_plus;
            count_plus = 0;
        } else {
            rs2 += diff;
            count_plus -= diff;
        }

        if rs1 > rs2 {
            // count_plus == 0
            let diff2 = rs1 - rs2;
            if diff2 > count_minus {
                rs1 -= count_minus;
                count_minus = 0;
                // count_plus == 0, count_minus == 0
                return rs2;
            } else {
                rs1 -= diff2;
                count_minus -= diff2;
                // rs1 == rs2
                // count_plus == 0, count_minus >= 0
                return rs1 - (count_minus + count_minus % 2) / 2;
            }
        }
    }

    // rs1 == rs2
    // count_plus >= 0, count_minus >= 0

    if count_plus > count_minus {
        count_minus = 0;
        count_plus -= count_minus;
        return rs1 + count_plus / 2;
    } else if count_plus < count_minus {
        count_plus = 0;
        count_minus -= count_plus;
        return rs1 - (count_minus + count_minus % 2) / 2;
    } else {
        return rs1;
    }
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
