use std::cmp::min;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let seeds = buf
        .trim_start_matches("seeds:")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    buf.clear();

    let mut raw_maps: Vec<Vec<(i64, i64, i64)>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim().ends_with("map:") {
            raw_maps.push(vec![]);
        } else if buf.trim().len() == 0 {
            // Do nothing.
        } else {
            let mut buf_iter = buf.trim().splitn(3, ' ').map(|s| s.parse::<i64>().unwrap());
            raw_maps.last_mut().unwrap().push((
                buf_iter.next().unwrap(),
                buf_iter.next().unwrap(),
                buf_iter.next().unwrap(),
            ));
        }
        buf.clear();
    }

    let maps: Vec<Vec<Item>> = create_maps(raw_maps);
    println!("{:?}", solve(seeds, maps));

    Ok(())
}

#[derive(Debug)]
struct Item {
    s0: i64,
    d0: i64,
    range: i64,
}

fn create_maps(raw_maps: Vec<Vec<(i64, i64, i64)>>) -> Vec<Vec<Item>> {
    raw_maps
        .iter()
        .map(|raw_items| {
            let mut items: Vec<Item> = raw_items
                .iter()
                .map(|&(d0, s0, range)| Item { s0, d0, range })
                .collect();
            items.sort_by_key(|item| item.s0);
            items
        })
        .collect()
}

fn search_map(value: i64, items: &[Item]) -> i64 {
    let mut l: i64 = 0;
    let mut r = items.len() as i64 - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        let item = &items[m as usize];
        if value >= item.s0 && value < item.s0 + item.range {
            return value - item.s0 + item.d0;
        } else if value < item.s0 {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    value
}

fn solve(seeds: Vec<i64>, maps: Vec<Vec<Item>>) -> i64 {
    let mut values = seeds.clone();

    for items in &maps {
        values = values.iter().map(|v| search_map(*v, items)).collect()
    }

    values.into_iter().reduce(|x, y| min(x, y)).unwrap()
}
