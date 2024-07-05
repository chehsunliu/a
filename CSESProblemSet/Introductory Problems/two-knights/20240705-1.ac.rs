use std::io::{self};

const ANSWERS: [i64; 3] = [0, 6, 28];

fn solve(n: i64) -> i64 {
    if n <= 3 {
        return ANSWERS[n as usize - 1];
    }

    let r0_r1_counts = [
        ((n - 1) / 4 + 1, (n - 3) / 4 + 1),
        ((n - 2) / 4 + 1, (n - 4) / 4 + 1),
    ];

    let mut answer = 0;

    for (r0_count, r1_count) in r0_r1_counts {
        let r0_r1_pairs = r0_count + r1_count - 1;
        let full_count = (n - 1) * r0_r1_pairs;
        answer += full_count * 4;
    }

    (n * n) * (n * n - 1) / 2 - answer
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n = buf.trim().parse::<i64>().unwrap();
        buf.clear();

        for nn in 1..=n {
            println!("{}", solve(nn));
        }
    }

    Ok(())
}

// (7-1)/4+1=2
// x _ _ _ x _ _
// _ _ x _ _ _ x
// x _ _ _ x _ _
// _ _ x _ _ _ x
// x _ _ _ x _ _
// _ _ x _ _ _ x
// x _ _ _ x _ _

// _ _ x _ _ _ x
// x _ _ _ x _ _
// _ _ x _ _ _ x
// x _ _ _ x _ _
// _ _ x _ _ _ x
// x _ _ _ x _ _
// _ _ x _ _ _ x

// x _ _ _ x _ _ _ x _
// _ _ x _ _ _ x _ _ _

// x _ _ _ x _ _ (7-1)/4+1=2
// _ x _ _ _ x _ (7-2)/4+1=2
// _ _ x _ _ _ x (7-3)/4+1=2
// _ _ _ x _ _ _ (7-4)/4+1=1

// x _ _ _ x _ _ _ (8-1)/4+1=2
// x _ _ _ x _ _ _ x (9-1)/4+1=3

// x _ x _ x _ x
// _ _ _ _ _ _ _
// _ x _ x _ x _
// _ _ _ _ _ _ _
// x _ x _ x _ x
// _ _ _ _ _ _ _
// _ x _ x _ x _

// x _ _
// _ _ x
// x _ _

// _ _ x
// x _ _
// _ _ x

// x _ x
// _ _ _
// _ x _
