use std::collections::{HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    let mut is_reading_first_section = true;
    let mut raw_stacks: Vec<String> = vec![];
    let mut raw_steps: Vec<String> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        if is_reading_first_section {
            if buf.trim().is_empty() {
                is_reading_first_section = false;
            } else {
                raw_stacks.push(buf.to_string());
            }
        } else {
            raw_steps.push(buf.trim().to_string());
        }

        buf.clear();
    }

    let mut stacks = parse_raw_stacks(raw_stacks);
    let steps = parse_raw_steps(raw_steps);

    for (count, (i_src, i_dst)) in steps {
        for _ in 0..count {
            let c = stacks[i_src].pop_back().unwrap();
            stacks[i_dst].push_back(c);
        }
    }

    let mut ans = "".to_string();
    for stack in stacks.iter() {
        ans.push(*stack.back().unwrap())
    }
    println!("{}", ans);

    Ok(())
}

fn parse_raw_stacks(mut raw_stacks: Vec<String>) -> Vec<VecDeque<char>> {
    let count = raw_stacks.pop().unwrap().split_whitespace().count();
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); count];

    for line in raw_stacks.iter().rev() {
        for offset in 0..count {
            let s = line[(offset * 4)..(offset * 4 + 3)].trim();
            if !s.is_empty() {
                stacks[offset].push_back(s.chars().nth(1).unwrap());
            }
        }
    }

    stacks
}

fn parse_raw_steps(raw_steps: Vec<String>) -> Vec<(usize, (usize, usize))> {
    let mut steps = vec![];

    for line in raw_steps.iter() {
        let (s1, s2) = line[5..].split_once(" ").unwrap();
        let count = s1.parse::<usize>().unwrap();

        let (s1, s2) = s2[5..].split_once(" ").unwrap();
        let index_src = s1.parse::<usize>().unwrap();
        let index_dst = s2[3..].parse::<usize>().unwrap();

        steps.push((count, (index_src - 1, index_dst - 1)));
    }

    steps
}
