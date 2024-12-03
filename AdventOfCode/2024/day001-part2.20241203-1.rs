use std::collections::HashMap;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut s1: HashMap<i32, i32> = HashMap::new();
    let mut s2: HashMap<i32, i32> = HashMap::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let tmp: Vec<&str> = buf.trim().split_whitespace().collect();
        let n1 = tmp[0].parse::<i32>().unwrap();
        let n2 = tmp[1].parse::<i32>().unwrap();

        s1.insert(n1, s1.get(&n1).unwrap_or(&0) + 1);
        s2.insert(n2, s2.get(&n2).unwrap_or(&0) + 1);

        buf.clear();
    }

    let mut ans = 0;

    for (k, v) in &s1 {
        ans += s2.get(k).unwrap_or(&0) * *v * *k;
    }

    println!("{}", ans);

    Ok(())
}
