use std::cmp::max;
use std::collections::HashMap;
use std::io::{self};

fn solve(mut movies: Vec<(i32, i32)>) -> i32 {
    movies.sort();
    let mut dp: HashMap<usize, i32> = HashMap::new();

    let mut ans = 0;
    for i in 0..movies.len() {
        ans = max(ans, f(i, &movies, &mut dp));
    }

    ans
}

fn f(index: usize, movies: &Vec<(i32, i32)>, dp: &mut HashMap<usize, i32>) -> i32 {
    if index >= movies.len() {
        return 0;
    }

    let m0 = movies[index];
    let l = movies.partition_point(|&m1| m1.0 < m0.1);
    if dp.contains_key(&l) {
        return 1 + dp.get(&l).unwrap();
    }

    let mut count = 0;

    for i in l..movies.len() {
        count = max(count, f(i, movies, dp));
    }

    dp.insert(l, count);
    count + 1
}

// 3---------------12
//     4--6 7---10    12-------15
//                    12-13 14-15

// 3-5
// 4-9
// 5-8
// 5-9
// 8-10

// 3-5 5-8 8-10
// 4-9
// 5-9

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
