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

    let (mut rs1, mut rs2) = if rs1 > rs2 { (rs1, rs2) } else { (rs2, rs1) };

    return if rs1 == rs2 {
        let boost = count_plus - count_minus;
        if boost >= 0 {
            rs1 + boost / 2
        } else {
            rs1 - (-boost) / 2 - (-boost) % 2
        }
    } else {
        // rs1 > rs2
        let mut diff = rs1 - rs2;
        if count_minus > diff {
            rs1 -= diff;
            count_minus -= diff;
            // now: rs1 == rs2, count_minus > 0, count_plus >= 0
            if count_minus > count_plus {
                count_minus -= count_plus;
                rs1 - count_minus / 2 - count_minus % 2
            } else if count_minus < count_plus {
                count_plus -= count_minus;
                rs1 + count_plus / 2
            } else {
                rs1
            }
        } else if count_minus < diff {
            rs1 -= count_minus;
            diff = rs1 - rs2;
            if count_plus > diff {
                count_plus -= diff;
                rs1 + count_plus / 2
            } else if count_plus < diff {
                rs2 + count_plus
            } else {
                rs1
            }
        } else {
            rs2 + count_plus / 2
        }
    };
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
