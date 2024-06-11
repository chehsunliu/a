use std::cmp::max;
use std::io::{self};

struct Movie {
    a: i32,
    b: i32,
}

fn solve(movies: &[Movie], current_i: usize) -> i32 {
    let mut next_i = None;
    for i in current_i..movies.len() {
        if movies[i].a >= movies[current_i].b {
            next_i = Some(i);
            break;
        }
    }

    match next_i {
        Some(next_i) => 1 + solve(movies, next_i),
        None => 1,
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    let mut movies: Vec<Movie> = vec![];

    for _ in 0..n {
        io::stdin().read_line(&mut buf)?;
        let mut buf_iter = buf.trim().splitn(2, ' ');
        let a = buf_iter.next().unwrap().parse::<i32>().unwrap();
        let b = buf_iter.next().unwrap().parse::<i32>().unwrap();
        buf.clear();

        movies.push(Movie { a, b });
    }

    movies.sort_by_key(|m| m.b);

    let mut ans = 0;
    for i in 0..movies.len() {
        ans = max(ans, solve(&movies, i));
    }

    println!("{}", ans);

    Ok(())
}
