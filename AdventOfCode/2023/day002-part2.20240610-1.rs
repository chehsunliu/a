use std::cmp::max;
use std::collections::HashMap;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut ans = 0;

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut buf_iter = buf.trim().splitn(2, ':').into_iter();
        parse_game_id(buf_iter.next().unwrap());
        let game_info = parse_game_info(buf_iter.next().unwrap());
        buf.clear();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cubes in game_info {
            let red = cubes.get("red").map_or(0, |c| *c);
            let green = cubes.get("green").map_or(0, |c| *c);
            let blue = cubes.get("blue").map_or(0, |c| *c);

            max_red = max(max_red, red);
            max_green = max(max_green, green);
            max_blue = max(max_blue, blue);
        }

        ans += max_red * max_green * max_blue;
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
