use std::io::{self, Read};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let (x, y) = buf.trim().split_once(' ').unwrap();
        let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        buf.clear();

        if y < -1 {
            println!("NO");
        } else {
            println!("YES");
        }
    }

    Ok(())
}
