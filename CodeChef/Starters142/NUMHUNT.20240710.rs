use std::io::{self};

fn solve(x: i64) -> i64 {
    if x == 1 {
        return 6;
    }
    let x1 = f(x);
    let x2 = f(x1 + 1);
    x1 * x2
}

fn f(mut x: i64) -> i64 {
    loop {
        let mut is_valid = true;
        for i in 2..((x as f64).sqrt() as i64 + 1) {
            if x % i == 0 {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            break;
        }
        x += 1;
    }

    x
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        let x = buf.trim().parse::<i64>().unwrap();
        buf.clear();
        println!("{}", solve(x));
    }

    Ok(())
}

// 3
// y has prime factors >= x
// 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
