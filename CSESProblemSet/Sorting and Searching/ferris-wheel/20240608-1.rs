use std::io::{self};

fn solve(x: i32, mut weights: Vec<i32>) -> i32 {
    weights.sort();

    let mut ans = 0;
    let mut l: i32 = 0;
    let mut r: i32 = weights.len() as i32 - 1;

    while l <= r {
        if weights[r as usize] > x {
            r -= 1;
            continue;
        }

        if weights[r as usize] + weights[l as usize] <= x {
            r -= 1;
            l += 1;
            ans += 1;
            continue;
        }

        ans += 1;
        r -= 1;
    }

    ans
}

fn split_i32s(s: &str, size: usize) -> Vec<i32> {
    s.trim()
        .splitn(size, ' ')
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let mut buf_iter = buf.trim().splitn(2, ' ');
    let n = buf_iter.next().unwrap().parse::<usize>().unwrap();
    let x = buf_iter.next().unwrap().parse::<i32>().unwrap();
    buf.clear();

    io::stdin().read_line(&mut buf)?;
    let weights = split_i32s(&buf, n);
    buf.clear();

    println!("{}", solve(x, weights));

    Ok(())
}
