use std::io::{self};

// q <= 10^18
fn solve(mut q: u64) -> u64 {
    let mut offset = 9;
    let mut offset_multiplier = 1;

    while q > offset * offset_multiplier {
        q -= offset * offset_multiplier;
        offset *= 10;
        offset_multiplier += 1;
    }

    let base = 10_u64.pow(offset_multiplier as u32 - 1);

    q -= 1;
    let mut value = base + (q / offset_multiplier);

    for _ in 0..(offset_multiplier - 1 - q % offset_multiplier) {
        value /= 10;
    }

    value % 10
}

// 9 / 2=4
// 9 % 2=1 10 + 4
// 123456789101112131415...9899100101102103...9991000...999910000
// ^       ^ -> 9*1
//          ^                 ^ -> 90*2
//                             ^                ^ -> 900*3
//                                               ^         ^ -> 9000*4

// 7 -> 7
// 19 -> 4
// 12 -> 1

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    while io::stdin().read_line(&mut buf)? != 0 {
        println!("{}", solve(buf.trim().parse::<u64>().unwrap()));
        buf.clear();
    }

    Ok(())
}
