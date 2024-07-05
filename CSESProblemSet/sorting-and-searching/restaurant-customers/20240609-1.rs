use std::cmp::max;
use std::collections::HashMap;
use std::io::{self};

fn solve(times: Vec<(i32, i32)>) -> i32 {
    let mut m: HashMap<i32, (i32, i32)> = HashMap::new();

    for (a, b) in times {
        let entry_a = m.entry(a).or_insert((0, 0));
        (*entry_a).0 += 1;

        let entry_b = m.entry(b).or_insert((0, 0));
        (*entry_b).1 += 1;
    }

    let mut ts: Vec<(i32, i32, i32)> = vec![];
    for k in m.keys() {
        let v = m.get(k).unwrap();
        ts.push((*k, v.0, v.1));
    }
    ts.sort();

    let mut max_consumers = 0;
    let mut consumers = 0;
    for (_, count_a, count_l) in &ts {
        consumers = consumers + count_a - count_l;
        max_consumers = max(max_consumers, consumers);
    }

    max_consumers
}

fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    let mut times: Vec<(i32, i32)> = vec![];
    for _ in 0..n {
        io::stdin().read_line(&mut buf)?;
        let mut buf_iter = buf.trim().splitn(2, ' ');
        let a: i32 = buf_iter.next().unwrap().parse().unwrap();
        let b: i32 = buf_iter.next().unwrap().parse().unwrap();
        times.push((a, b));
        buf.clear();
    }

    println!("{}", solve(times));

    Ok(())
}
