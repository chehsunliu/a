use std::collections::HashMap;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut buf_iter = buf.trim().splitn(2, ':').into_iter();
        let game_id = parse_game_id(buf_iter.next().unwrap());
        let game_info = parse_game_info(buf_iter.next().unwrap());
        buf.clear();

        let mut is_possible = true;

        for cubes in game_info {
            is_possible &= cubes.get("red").map_or(0, |c| *c) <= 12;
            is_possible &= cubes.get("green").map_or(0, |c| *c) <= 13;
            is_possible &= cubes.get("blue").map_or(0, |c| *c) <= 14;
        }

        if is_possible {
            ans += game_id;
        }
    }

    println!("{}", ans);

    Ok(())
}

fn parse_game_id(raw: &str) -> i32 {
    raw[5..].parse().unwrap()
}

fn parse_game_info(raw: &str) -> Vec<HashMap<String, i32>> {
    let mut game_info = vec![];

    for raw_set in raw.split(';') {
        let mut cubes: HashMap<String, i32> = HashMap::new();

        for raw_cubes in raw_set.trim().split(", ") {
            let mut raw_cubes_iter = raw_cubes.splitn(2, ' ');
            let count: i32 = raw_cubes_iter.next().unwrap().parse().unwrap();
            let color = raw_cubes_iter.next().unwrap();
            cubes.insert(color.to_string(), count);
        }

        game_info.push(cubes);
    }

    game_info
}
