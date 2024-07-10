use std::collections::HashSet;
use std::io::{self};

fn solve(mut arr: Vec<u32>, valid_values: &HashSet<u32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    arr.sort();
    let mut acc = 0;
    let mut valid_i = None;
    for (i, v) in arr.iter().enumerate() {
        acc |= *v;
        if valid_values.contains(&acc) {
            valid_i = Some(i as i32);
        }
    }

    if let Some(i) = valid_i {
        arr.len() as i32 - 1 - i
    } else {
        arr.len() as i32
    }
}

// > 00001 ()
// > 10001
// > 10010
// > 00101
// > 00110
// = 10111

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.clear();

    let mut valid_values: HashSet<u32> = HashSet::new();
    for i in 0..32 {
        valid_values.insert((1 << i) - 1);
    }

    while io::stdin().read_line(&mut buf)? != 0 {
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let arr = buf
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        buf.clear();
        println!("{}", solve(arr, &valid_values));
    }

    Ok(())
}
