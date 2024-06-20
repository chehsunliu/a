use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut cards: Vec<Card> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let mut s_iter = buf.trim().splitn(2, ": ");
        s_iter.next();

        let s = s_iter.next().unwrap();
        let mut s_iter = s.trim().splitn(2, " | ");

        let s1 = s_iter.next().unwrap();
        let s2 = s_iter.next().unwrap();

        let winning_nums = s1
            .split_whitespace()
            .map(|r| r.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let nums = s2
            .split_whitespace()
            .map(|r| r.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        buf.clear();

        cards.push(Card {
            winning_nums: HashSet::from_iter(winning_nums),
            nums: HashSet::from_iter(nums),
        })
    }

    println!("{}", solve(cards));

    Ok(())
}

struct Card {
    winning_nums: HashSet<i32>,
    nums: HashSet<i32>,
}

fn solve(cards: Vec<Card>) -> i32 {
    let mut counts: Vec<usize> = vec![0; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let count = card.winning_nums.intersection(&card.nums).count();
        counts[i] += 1;
        for j in 1..(count + 1) {
            counts[i + j] += counts[i];
        }
    }

    counts.iter().sum::<usize>() as i32
}
