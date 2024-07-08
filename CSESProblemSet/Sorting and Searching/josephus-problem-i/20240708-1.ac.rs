use std::io::{self};

fn find_the_winner(n: i32, k: i32) -> Vec<i32> {
    let mut is_out = vec![false; n as usize];
    let mut i = 0;
    let mut ans = vec![];

    for _ in 0..(n - 1) {
        let mut kk = 0;
        while kk < k - 1 {
            i = (i + 1) % n;
            if is_out[i as usize] {
                continue;
            }
            kk += 1;
        }

        is_out[i as usize] = true;
        ans.push(i);
        i = (i + 1) % n;
        while is_out[i as usize] {
            i = (i + 1) % n;
        }
    }

    let v = is_out
        .iter()
        .enumerate()
        .filter(|&(_, &out)| !out)
        .last()
        .unwrap()
        .0 as i32;
    ans.push(v);
    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let result = find_the_winner(buf.trim().parse::<i32>().unwrap(), 2);
    println!(
        "{}",
        result
            .into_iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    buf.clear();

    Ok(())
}
