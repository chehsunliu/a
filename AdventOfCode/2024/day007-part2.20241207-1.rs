// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut ans = 0;

    while std::io::stdin().read_line(&mut buf)? != 0 {
        let (target, values) = buf.trim().split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let values = values
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        if solve(target, 0, &values) {
            ans += target;
        }

        buf.clear();
    }

    println!("{}", ans);

    Ok(())
}

fn solve(target: u64, acc: u64, values: &[u64]) -> bool {
    if values.is_empty() {
        return acc == target;
    }

    if acc > target {
        return false;
    }

    solve(target, acc + values[0], &values[1..])
        || solve(target, acc * values[0], &values[1..])
        || solve(
            target,
            acc * 10u64.pow(digits(values[0])) + values[0],
            &values[1..],
        )
}

fn digits(mut v: u64) -> u32 {
    let mut d = 0;
    while v > 0 {
        v /= 10;
        d += 1;
    }
    d.max(1)
}
