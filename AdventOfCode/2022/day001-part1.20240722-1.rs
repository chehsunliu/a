use std::cmp::max;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut ans = 0;
    let mut local_ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim().is_empty() {
            ans = max(ans, local_ans);
            local_ans = 0;
        } else {
            local_ans += buf.trim().parse::<i32>().unwrap();
        }
        buf.clear();
    }
    ans = max(ans, local_ans);

    println!("{ans}");

    Ok(())
}
