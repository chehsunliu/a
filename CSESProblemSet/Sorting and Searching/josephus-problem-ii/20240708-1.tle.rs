use std::collections::VecDeque;
use std::io::{self};

fn find_the_winner(n: i32, k: i32) -> Vec<i32> {
    let mut ans = vec![];

    let mut queue = VecDeque::new();
    for v in 1..=n {
        queue.push_back(v);
    }

    while !queue.is_empty() {
        for _ in 0..(k % queue.len() as i32) {
            let v = queue.pop_front().unwrap();
            queue.push_back(v);
        }

        ans.push(queue.pop_front().unwrap());
    }

    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let (n, k) = buf.trim().split_once(' ').unwrap();
    let (n, k) = (n.parse::<i32>().unwrap(), k.parse::<i32>().unwrap());
    buf.clear();

    let result = find_the_winner(n, k)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);

    Ok(())
}
