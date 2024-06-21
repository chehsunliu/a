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
    println!("{:?}", solve(seeds, &maps));

    Ok(())
}

#[derive(Debug)]
struct Item {
    s: (i64, i64),
    d: (i64, i64),
}

fn create_maps(raw_maps: Vec<Vec<(i64, i64, i64)>>) -> Vec<Vec<Item>> {
    raw_maps
        .iter()
        .map(|raw_items| {
            let mut items: Vec<Item> = raw_items
                .iter()
                .map(|&(d0, s0, range)| Item {
                    s: (s0, s0 + range - 1),
                    d: (d0, d0 + range - 1),
                })
                .collect();
            items.sort_by_key(|item| item.s.0);
            items
        })
        .collect()
}

fn search_map(mut values: Vec<(i64, i64)>, items: &[Item]) -> Vec<(i64, i64)> {
    let mut i = 0;
    let mut ans = vec![];

    while i < values.len() {
        let (s0, s1) = values[i];
        // println!("{} {}", s0, s1);

        let mut l: i32 = 0;
        let mut r: i32 = items.len() as i32 - 1;
        let mut found = false;

        while l <= r {
            let m = l + (r - l) / 2;
            let item = &items[m as usize];

            if s1 < item.s.0 {
                r = m - 1;
            } else if item.s.1 < s0 {
                l = m + 1;
            } else {
                if item.s.0 <= s0 && s1 <= item.s.1 {
                    ans.push((item.d.0 + s0 - item.s.0, item.d.0 + s1 - item.s.0))
                } else if item.s.0 > s0 && s1 <= item.s.1 {
                    ans.push((item.d.0, item.d.0 + s1 - item.s.0));
                    values.push((s0, item.s.0 - 1));
                } else if item.s.0 <= s0 && s1 > item.s.1 {
                    ans.push((item.d.0 + s0 - item.s.0, item.d.1));
                    values.push((item.s.1 + 1, s1));
                } else {
                    ans.push((item.d.0, item.d.1));
                    values.push((s0, item.s.0 - 1));
                    values.push((item.s.1 + 1, s1));
                }
                found = true;
                break;
            }
        }

        if !found {
            ans.push((s0, s1));
        }

        i += 1;
    }

    ans
}

fn solve(seeds: Vec<i64>, maps: &Vec<Vec<Item>>) -> i64 {
    let mut values: Vec<(i64, i64)> = vec![];
    for i in (0..seeds.len()).step_by(2) {
        values.push((seeds[i], seeds[i] + seeds[i + 1] - 1));
    }

    for items in maps {
        values = search_map(values, items);
    }

    let mut ans = i64::MAX;
    for (s0, _) in values {
        ans = min(ans, s0);
    }

    ans
}
