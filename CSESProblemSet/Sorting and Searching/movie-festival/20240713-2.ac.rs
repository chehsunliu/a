use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::io::{self};

fn solve(mut movies: Vec<(i32, i32)>) -> i32 {
    movies.sort();

    let mut ans = 0;
    let mut dp = vec![-1; movies.len()];

    for i in (0..movies.len()).rev() {
        let mut queue = VecDeque::new();
        queue.push_back((i, 1));

        let mut local_ans = 0;
        while let Some((index, count)) = queue.pop_back() {
            let m0 = movies[index];
            let l = movies.partition_point(|&m1| m1.0 < m0.1);

            if l < dp.len() && dp[l] != -1 {
                local_ans = max(local_ans, count + dp[l]);
            } else {
                for j in l..movies.len() {
                    queue.push_back((j, count + 1));
                }
                local_ans = max(local_ans, count);
            }
        }

        dp[i] = max(ans, local_ans);
        ans = max(ans, local_ans);
    }

    ans
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
