use std::cmp::max;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut ans = vec![0];

    while io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim().is_empty() {
            ans.push(0);
        } else {
            ans.last_mut()
                .map(|v| *v += buf.trim().parse::<i32>().unwrap());
        }
        buf.clear();
    }

    ans.sort_by(|a, b| b.cmp(a));

    println!("{}", ans[..3].iter().sum::<i32>());

    Ok(())
}
