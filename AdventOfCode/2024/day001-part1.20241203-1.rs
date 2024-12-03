use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut v1 = vec![];
    let mut v2 = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let tmp: Vec<&str> = buf.trim().split_whitespace().collect();
        v1.push(tmp[0].parse::<i32>().unwrap());
        v2.push(tmp[1].parse::<i32>().unwrap());
        buf.clear();
    }

    v1.sort();
    v2.sort();

    let mut distance = 0;
    for (n1, n2) in v1.iter().zip(v2.iter()) {
        distance += (n1 - n2).abs();
    }

    println!("{}", distance);

    Ok(())
}
