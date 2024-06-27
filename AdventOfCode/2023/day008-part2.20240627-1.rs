use std::collections::HashMap;
use std::io::{self, Read};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut buf_iter = buf.lines();
    let instructions = buf_iter.next().unwrap().chars().collect::<Vec<char>>();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in buf_iter {
        if line.trim().is_empty() {
            continue;
        }

        // KCN = (FCB, RMP)
        let (s0, s1) = line.split_once('=').unwrap();
        let (s0, s1) = (s0.trim(), s1.trim());
        let (s2, s3) = s1[1..s1.len() - 1].split_once(',').unwrap();
        let (s2, s3) = (s2.trim(), s3.trim());
        map.insert(s0, (s2, s3));
    }

    let mut total_steps = 1;

    for node in map.keys().map(|&k| k).filter(|&k| k.ends_with('A')) {
        let mut steps = 0;
        let mut instruction_index = 0;
        let mut node = node;

        while !node.ends_with('Z') {
            let instruction = instructions[instruction_index];
            let possible_nodes = *map.get(node).unwrap();
            node = if instruction == 'L' {
                possible_nodes.0
            } else {
                possible_nodes.1
            };

            steps += 1;
            instruction_index = (instruction_index + 1) % instructions.len();
        }

        total_steps = total_steps / gcd(total_steps, steps) * steps;
    }

    println!("{total_steps}");

    Ok(())
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
