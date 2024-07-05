use std::collections::HashSet;
use std::io::{self};

fn ff(arr: &[i32], i: usize, sum: i32, sums: &mut HashSet<i32>) {
    if i >= arr.len() {
        sums.insert(sum);
        return;
    }

    ff(arr, i + 1, sum, sums);
    ff(arr, i + 1, sum + arr[i], sums);
}

fn solve(mut arr: Vec<i32>) -> i32 {
    arr.sort();

    let mut sums: HashSet<i32> = HashSet::new();
    ff(arr.as_slice(), 0, 0, &mut sums);

    let mut ans = 1;
    while sums.contains(&ans) {
        ans += 1;
    }

    ans
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let n: usize = buf.trim().parse().unwrap();
        buf.clear();

        io::stdin().read_line(&mut buf)?;
        let arr = buf
            .trim()
            .splitn(n, ' ')
            .map(|token| token.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        buf.clear();

        println!("{}", solve(arr));
    }

    Ok(())
}
