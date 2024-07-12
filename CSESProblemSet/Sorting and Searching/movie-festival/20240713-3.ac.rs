use std::cmp::max;
use std::io::{self};

fn solve(mut movies: Vec<(i32, i32)>) -> i32 {
    movies.sort();

    let mut dp = vec![0; movies.len() + 1];

    for i in (0..movies.len()).rev() {
        let m0 = movies[i];
        let l = movies.partition_point(|&m1| m1.0 < m0.1);

        if l >= dp.len() {
            dp[i] = dp[i + 1];
        } else {
            dp[i] = max(dp[i + 1], 1 + dp[l]);
        }
    }

    dp[0]
}

// 201---255 v
//          404------------------------------------------882
//                457--------601 v (3)
//                  461------------------------------------------978 (1)
//                       525------------------819 (2)
//                              690-----------------------------974 (1)
//                                699-------804 v (2)
//                                   795------------860 (1)
//                                      800--------------------933 (1)
//                                               832--------932 v (1)

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    buf.clear();

    let mut movies = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let (t0, t1) = buf.trim().split_once(' ').unwrap();
        let (t0, t1) = (t0.parse::<i32>().unwrap(), t1.parse::<i32>().unwrap());
        movies.push((t0, t1));
        buf.clear();
    }

    println!("{}", solve(movies));

    Ok(())
}
