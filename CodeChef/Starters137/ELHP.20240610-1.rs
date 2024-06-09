use std::io::{self};

fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut buf_iter = buf.trim().splitn(2, ' ');
        let x: i32 = buf_iter.next().unwrap().parse().unwrap();
        let y: i32 = buf_iter.next().unwrap().parse().unwrap();
        buf.clear();

        if x >= y * 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    Ok(())
}
